use highlighter::highlighter::HighlighterConfig;

use crate::{document::Document, position::Position, selection::{Range, SelectionList}};

pub struct Buffer {
    document: Document,
    selections: SelectionList,
    highlighter: Option<HighlighterConfig>,
    // window_height: usize
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

    pub fn new(document: Document, selections: SelectionList) -> Self {
        Self {
            document,
            selections,
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
            let document_change = DocumentChange::Delete(selection.range());
            self.doc_change(document_change);
        }
    }

    pub fn insert(&mut self, value: String) {
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

    pub fn get_selected(&self) -> String {
        String::new()
    }

    pub fn copy(&mut self) {

    }

    pub fn cut(&mut self) {

    }

    pub fn paste(&mut self) {

    }

    pub fn move_cursor(&mut self) {
        // All movement subcribers can be notified here.
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