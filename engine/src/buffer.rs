use crate::{document::{self, Document}, selection::Selection, tab::Tab, Cursor};

pub struct Buffer {
    document: Document,
    cursor: Cursor,
    selection: Selection,
}

impl Buffer {
    pub fn open(file: &str) -> Self {
        Self {
            document: Document::open(file).unwrap(),
            cursor: Cursor::default(),
            selection: Selection::default()
        }
    }

    pub fn new(document: Document, cursor: Cursor, selection: Selection) -> Self {
        Self {
            document,
            cursor,
            selection
        }
    }

    pub fn delete(&mut self) {

    }

    pub fn insert(&mut self) {

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

impl From<Buffer> for Tab {
    fn from(value: Buffer) -> Self {
        Tab::new(value.document, value.cursor)
    }
}