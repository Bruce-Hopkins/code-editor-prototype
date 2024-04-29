use std::usize;

use highlighter::highlighter::{HighlighterConfig, Highlighter, HighlightIter};
use ropey::RopeSlice;
use tree_sitter::InputEdit;

use crate::{document::{ByteRange, Document, DocumentPositionIter}, position::{self, Position}, selection::{Range, Selection, SelectionList}, undo::{self, Undo, UndoItem}};

#[derive(Default)]
pub struct Buffer {
    document: Document,
    selections: SelectionList,
    highlighter: Option<HighlighterConfig>,
    undo: Undo,
    window_height: usize
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.document.to_string()
    }
}

enum DocumentChange {
    Insert(String, Position),
    Delete(Range),
    Replace(String, Range)
}

struct HighlighterChange {
    start_byte: usize,
    end_byte: usize,
    new_end_byte: usize,
    start_pos: Position,
    end_pos: Position,
    new_end_pos: Position,
}

impl HighlighterChange{
    fn from_insert(pos: usize, cursor: Position, len: usize) -> Self {
        Self {
            start_byte: pos,
            end_byte: pos,
            new_end_byte: pos.saturating_add(len),
            end_pos: cursor,
            start_pos: cursor,
            new_end_pos: Position {character: cursor.character.saturating_add(len), ..cursor},
        }
    }

    fn from_delete(byte_range: ByteRange, range: Range) -> Self {
        Self { 
            start_byte: byte_range.start.saturating_add(1),
            end_byte: byte_range.end,
            new_end_byte: byte_range.start,
            start_pos: Position{character: range.start().character.saturating_add(1), ..range.start()},
            end_pos: range.end(),
            new_end_pos: range.end(),
        }
    }

    fn from_replace(byte_range: ByteRange, range: Range, len: usize) -> Self {
        Self { 
            start_byte: byte_range.start,
            end_byte: byte_range.end,
            new_end_byte: byte_range.start.saturating_add(len),
            end_pos: range.start(),
            start_pos: range.end(),
            new_end_pos: Position {character: range.start().character.saturating_add(len), ..range.start()}
        }
    }
}

impl Into <tree_sitter::Point> for Position {
    fn into(self) -> tree_sitter::Point {
        tree_sitter::Point {
            row: self.line,
            column: self.character
        }
    }
}

impl Into <tree_sitter::InputEdit> for HighlighterChange {
    fn into(self)  -> InputEdit {
        InputEdit { 
            start_byte: self.start_byte, 
            old_end_byte: self.end_byte, 
            new_end_byte: self.new_end_byte, 
            start_position: self.start_pos.into(), 
            old_end_position: self.end_pos.into(), 
            new_end_position: self.new_end_pos.into() 
        }
    }
}

impl Buffer {
    pub fn open(file: &str) -> Self {
        // TODO, if file extension matches the highlighter configs, create a new highlighter.
        Self {
            document: Document::open(file).unwrap(),
            selections: SelectionList::default(),
            highlighter: None,
            window_height: 0,
            undo: Undo::new(100),
        }
    }
    

    pub fn highlighter<'a>(&'a self) -> Option<Highlighter<'a>> {
        if let Some(highlighter) = self.highlighter.as_ref() {
            return Some(Highlighter::new(highlighter, 0..usize::MAX, 
                &self.document
            ))
        }
        None
    }

    pub fn set_highlighter(&mut self, highlighter: HighlighterConfig) {
        self.highlighter = Some(highlighter);
    }

    pub fn set_selections(&mut self, list: SelectionList) {
        self.selections = list;
    }

    // fn get_selected_text(&self) -> Vec<String> {
    //     let mut selected_text = Vec::new();
    //     for selection in self.selections.get_all() {
    //         let range = selection.range();
    //         let start_bytes = self.document.get_line_bytes(range.start().line) + range.end().character;
    //         let end_bytes = self.document.get_line_bytes(selection.end().line) + selection.end().character;
    
    //         let slice = self.document.str_from_range(start_bytes, end_bytes);
    //         selected_text.push(slice.to_string());
    //     }
    //     selected_text
    // }

    fn get_text_in_range(&self, range: &Range) -> String {
        let start_bytes = self.document.get_line_bytes(range.start().line) + range.start().character;
        let end_bytes = self.document.get_line_bytes(range.end().line) + range.end().character;

        let slice = self.document.str_from_range(start_bytes, end_bytes);
        slice.to_string()
    }

    pub fn doc_iter<'document>(&'document self, pos: Position) -> DocumentPositionIter {
        DocumentPositionIter::new(pos, &self.document)
    }

    /**
     * A central method for all document changes.
     */
    fn document_edit(&mut self, document_change: DocumentChange, should_undo: bool) {
        match document_change {
            DocumentChange::Insert(value, pos) => {
                let insert_len = value.len();
                let bytes = self.document.insert(&pos, value.clone());

                if let Some(bytes) = bytes {

                    if should_undo {
                        self.undo.insert(value, pos);
                    }
                    if let Some(highlighter) = self.highlighter.as_mut() {
    
                        let highlighter_change = HighlighterChange::from_insert(bytes, pos, insert_len);
                        highlighter.edit(&highlighter_change.into(), &self.document.slice_all());
                    }
                }

            },
            DocumentChange::Delete(range) => {
                let doomed_text = if should_undo {
                    self.get_text_in_range(&range)
                } else {
                    String::new()
                };

                let bytes = self.document.delete(&range);

                match bytes {
                    Ok(bytes) => {
                        if let Some(bytes) = bytes {
                            if should_undo {
                                self.undo.delete(doomed_text, range);
                            } 

                            if let Some(highlighter) = self.highlighter.as_mut() {
                                let highlighter_change = HighlighterChange::from_delete(bytes, range);
                                highlighter.edit(&highlighter_change.into(), &self.document.slice_all())
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to run delete in the document {:?}", e);
                    }
                }
            },
            DocumentChange::Replace(value, range) => {
                let insert_len = value.len();
                let doomed_text = if should_undo {
                     self.get_text_in_range(&range)
                } else {
                    String::new()
                };
                

                let bytes = self.document.replace(&range, value.clone());
                match bytes {
                    Ok(bytes) => {
                        if let Some(bytes) = bytes {
                            if should_undo {
                                self.undo.replace(value, doomed_text, range);
                            }
                            if let Some(highlighter) = self.highlighter.as_mut() {    
                                let highlighter_change = HighlighterChange::from_replace(bytes, range, insert_len);
                                highlighter.edit(&highlighter_change.into(), &self.document.slice_all())
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to run replace in the document {:?}", e);
                    }
                }
            },
        }
    }

    pub fn delete(&mut self) {
        for selection in self.selections.get_all() {
            let mut range = selection.range();
            if range.is_same() {
                range = Range::new(range.start(), Position::new(range.end().line, range.end().character + 1) )
            }
            let document_change = DocumentChange::Delete(range);
            self.document_edit(document_change, true);
        }
    }

    /**
     * Inserts the passed text into all of the selections and cursors
     */
    pub fn insert<T>(&mut self, value: &T)
    where T: ToString + 'static {
        let value = value.to_string();
        for (i, selection) in self.selections.get_all().into_iter().enumerate() {
            if selection.is_empty() {
                let document_change = DocumentChange::Insert(value.clone(), selection.end());

                let selection = self.selections.get_mut(i).unwrap();
                selection.move_cursor_to_end_of_insert(&value);

                self.document_edit(document_change, true);
            }
            else {
                let range = selection.range();
                let document_change = DocumentChange::Replace(value.clone(), range);

                let selection = self.selections.get_mut(i).unwrap();
                selection.move_cursor_to_end_of_insert(&value);

                self.document_edit(document_change, true);
            }
        }
    }

    /**
     * Returns a vector of selections
     */

    pub fn get_selection(&self) -> Vec<Selection> {
        self.selections.get_all()
    }

    /**
     * Get's the byte number at the position
     */
    pub fn get_character_inx_from_position(&self, position: &Position) -> Option<usize> {
        self.document.get_character_pos(position)
    }

    pub fn get_slice_from_pos(&self, position: &Position) -> Option<RopeSlice>  {
        let character_idx = self.document.get_character_pos(position)?;
        self.document.get_str_from_range(character_idx, character_idx.saturating_add(1))
    }

    // pub fn get_selected(&self) -> String {
    //     String::new()
    // }

    pub fn copy(&mut self) {

    }

    pub fn cut(&mut self) {

    }

    pub fn paste(&mut self) {

    }

    pub fn is_on_tab_line(&mut self) {
        todo!()
    }

    // /**
    //  * Peaks what the 
    //  */
    // pub fn peak(&self, len: usize) {

    // }

    /**
     * Moves one cursor to the selected location.
     * 
     * If there are any selections, it will delete those selections.
     */
    pub fn move_cursor(&mut self, pos: Position) {
        self.selections.replace_cursor(pos);
    }

    /**
     * Moves the selection of the last moved cursor. 
     * */ 
    pub fn move_selection(&mut self, pos: Position) {
        self.selections.move_selection(pos)
    }

    /**
     * Adds a new cursor with the given position.
     */
    pub fn add_cursor(&mut self, pos: Position) {
        self.selections.push(pos);
    }

    /**
     * Redos the last change in the redo stack
     */
    pub fn redo(&mut self) {
        if let Some(redo_item) = self.undo.redo() {
            self.reverse_doc_change(redo_item);
        }
    }

    fn reverse_doc_change(&mut self, undo_item: UndoItem) {
        match undo_item {
            undo::UndoItem::Insert(position, text) => {
                let doc_change = DocumentChange::Insert(text, position);
                self.document_edit(doc_change, false);
            },
            undo::UndoItem::Delete(position) => {
                let doc_change = DocumentChange::Delete(position);
                self.document_edit(doc_change, false)
            },
            undo::UndoItem::Replace(range, text) => {
                let doc_change = DocumentChange::Replace(text, range);
                self.document_edit(doc_change, false);
            }
        }
    }

    /**
     * Undos the last change in the undo stack
     */
    pub fn undo(&mut self) {
        if let Some(undo_item) = self.undo.undo() {
            self.reverse_doc_change(undo_item);
        }
    }

    pub fn get_line_len(&self) {

    }

    pub fn correct_cursor(&mut self) {
        
    }

}

// impl From<Buffer> for Tab {
//     fn from(value: Buffer) -> Self {
//         Tab::new(value.document, value.cursors, value.selections)
//     }
// }
