use crate::{buffer::Buffer, document::Document, position::{Cursor, CursorList}, selection::{self, Selection, SelectionList}};

pub struct Tab {
    document: Document,
    cursors: CursorList,
    selections: SelectionList
}

impl Tab {
    pub fn new(document: Document, cursors: CursorList, selections: SelectionList) -> Tab {
        Tab {
            document,
            cursors,
            selections,
        }
    }
}

impl From<Tab> for Buffer {
    fn from(value: Tab) -> Self {
        Buffer::new(value.document, value.cursors, value.selections)
    }
}