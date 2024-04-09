use crate::{buffer::Buffer, document::Document, selection::{self, Selection}, Cursor};

pub struct Tab {
    document: Document,
    cursor: Cursor
}

impl Tab {
    pub fn new(document: Document, cursor: Cursor) -> Tab {
        Tab {
            document,
            cursor
        }
    }
}

impl From<Tab> for Buffer {
    fn from(value: Tab) -> Self {
        Buffer::new(value.document, value.cursor, Selection::default())
    }
}