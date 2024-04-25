// use crate::{buffer::Buffer, document::Document, position::{Cursor, SelectionList}, selection::{self, Selection, SelectionList}};

// pub struct Tab {
//     document: Document,
//     cursors: SelectionList,
//     selections: SelectionList
// }

// impl Tab {
//     pub fn new(document: Document, cursors: SelectionList, selections: SelectionList) -> Tab {
//         Tab {
//             document,
//             cursors,
//             selections,
//         }
//     }
// }

// impl From<Tab> for Buffer {
//     fn from(value: Tab) -> Self {
//         Buffer::new(value.document, value.cursors, value.selections)
//     }
// }