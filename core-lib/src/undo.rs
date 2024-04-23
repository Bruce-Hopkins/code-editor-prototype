use std::collections::VecDeque;

use crate::position::{self, Position};
use crate::selection::Range;

pub struct Undo {
    undo_stack: UndoStack,
    redo_stack: UndoStack,
    current_item: Option<TextChange>
}


impl Undo {

    pub fn new(capacity: usize) -> Self {
        Undo {
            undo_stack: UndoStack::new(capacity),
            redo_stack: UndoStack::new(capacity),
            current_item: None
        }
    }

    pub fn add(&mut self, text:String, position: Position) {
        self.undo_stack.push(TextChange::Insert(position, text));
    }
    
    fn replace_current_item(&mut self, change:TextChange) {
        if let Some(current_item) = self.current_item.as_mut() {

            if let (TextChange::Insert(current_item_pos, current_item_text), TextChange::Insert(position, change_text)) = (current_item, &change) {
                current_item_text.push_str(change_text);
                let pos= 
                return
            }
            else if let (TextChange::Delete(_, _), TextChange::Delete(_, _)) = (current_item, &change) {
                
            }
            else if let (TextChange::Replace(_, _), TextChange::Replace(_, _)) = (current_item, &change) {
                
            }            
        }
        self.current_item = Some(change)
    }

    pub fn delete(&mut self, text:String, range: Range) {
        self.undo_stack.push(TextChange::Delete(range, text))
    }

    pub fn replace(&mut self, text:String, range: Range) {
        self.undo_stack.push(TextChange::Replace(range, text))
    }

    pub fn push(&mut self) {
        todo!()
    }

    pub fn redo(&mut self) -> TextChange {
        todo!()
    }

    pub fn undo(&mut self) -> TextChange {
        todo!()
    }
}

struct UndoStack(VecDeque<TextChange>);
impl  UndoStack{

    fn new(capacity:usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    fn push(&mut self, change:TextChange) {
        todo!()
    }
}

pub enum TextChange {
    Insert(Position,String),
    Delete(Range,String),
    Replace(Range,String)
}

impl TextChange {
    fn reverse(&mut self) {
        todo!()
    }
}
