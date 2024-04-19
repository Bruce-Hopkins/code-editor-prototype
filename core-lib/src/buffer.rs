use highlighter::highlighter::HighlighterConfig;

use crate::{document::Document, position::Position, selection::{Range, Selection, SelectionList}};

#[derive(Default)]
pub struct Buffer {
    document: Document,
    selections: SelectionList,
    highlighter: Option<HighlighterConfig>,
    // window_height: usize
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

impl Buffer {
    pub fn open(file: &str) -> Self {
        // TODO, if file extension matches the highlighter configs, create a new highlighter.
        Self {
            document: Document::open(file).unwrap(),
            selections: SelectionList::default(),
            highlighter: None
        }
    }

    /**
     * A central method for all document changes.
     */
    fn doc_change(&mut self, document_change: DocumentChange) {
        match document_change {
            DocumentChange::Insert(value, pos) => {
                let bytes = self.document.insert(&pos, value);
            },
            DocumentChange::Delete(range) => {
                let bytes = self.document.delete(&range);
            },
            DocumentChange::Replace(value, range) => {
                let bytes = self.document.replace(&range, value);
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
            self.doc_change(document_change);
        }
    }

    /**
     * Inserts the passed text into all of the selections and cursors
     */
    pub fn insert<T>(&mut self, value: &T)
    where T: ToString + 'static {
        let value = value.to_string();
        for selection in self.selections.get_all() {
            if selection.is_empty() {
                let document_change = DocumentChange::Insert(value.clone(), selection.end());
                self.doc_change(document_change);
            }
            else {
                let range = selection.range();
                let document_change = DocumentChange::Replace(value.clone(), range);

                self.doc_change(document_change);
            }
        }
    }

    /**
     * Returns a vector of selections
     */

    pub fn get_selection(&self) -> Vec<Selection> {
        self.selections.get_all()
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