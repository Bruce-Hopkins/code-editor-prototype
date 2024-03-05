use super::{document::ByteRange, selection::Range};

pub struct DocumentChange {
    pub range: Range,
    pub byte_range: ByteRange,
    pub text: String,
    pub file: String
}

impl DocumentChange {
    pub fn new(range: Range, byte_range: ByteRange, text: String, file:String) -> Self {
        Self {
            range,
            byte_range,
            text,
            file,
        }
    }
}
