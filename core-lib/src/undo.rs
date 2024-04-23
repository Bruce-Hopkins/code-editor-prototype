use std::collections::VecDeque;

use crate::position::Position;
use crate::selection::Range;

struct Undo {
    undo_stack: TextStack,
    redo_stack: TextStack
}

struct TextStack(VecDeque<TextChange>);

impl Undo {
    pub fn add() {
        todo!()
    }

    pub fn push() {
        todo!()
    }

    pub fn redo() {
        todo!()
    }

    pub fn undo() {
        todo!()
    }
}

enum TextChange {
    Insert {
        position: Position,
        text: String
    },
    Delete {
        position: Range,
        text: String,
    },
    Replace {
        position: Range,
        text: String
    }
}

impl TextChange {
    fn reverse(&self) {
        todo!()
    }
}
