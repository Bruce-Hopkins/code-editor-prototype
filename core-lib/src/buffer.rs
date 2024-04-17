use highlighter::highlighter::HighlighterConfig;

use crate::{document::Document, position::{Cursor, CursorList}, selection::{Selection, SelectionList}, tab::Tab};

pub struct Buffer {
    document: Document,
    selections: SelectionList,
    cursors: CursorList,
    highlighter: Option<HighlighterConfig>,
    // window_height: usize
}

impl Buffer {
    pub fn open(file: &str) -> Self {
        Self {
            document: Document::open(file).unwrap(),
            cursors: CursorList::default(),
            selections: SelectionList::default(),
            highlighter: None
        }
    }

    pub fn new(document: Document, cursors: CursorList, selections: SelectionList) -> Self {
        Self {
            document,
            selections,
            cursors,
            highlighter: None
        }
    }

    pub fn delete(&mut self) {
        
    }

    pub fn insert(&mut self, value: String) {
        
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
        Tab::new(value.document, value.cursors, value.selections)
    }
}