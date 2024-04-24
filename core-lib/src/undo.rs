use std::collections::VecDeque;
use std::result;

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
        self.replace_current_item(TextChange::Insert(position, text));
        self.undo_stack.clear();
    }
    
    fn replace_current_item(&mut self, change:TextChange) {
        if let Some(current_item) = self.current_item.as_mut() {

            if let (TextChange::Insert(current_item_pos, current_item_text), TextChange::Insert(_, change_text)) = (current_item.as_mut(), &change) {
                current_item_text.push_str(change_text);
                let end_of_insert_pos = Position::end_of_insert(change_text);
                current_item_pos.move_by(end_of_insert_pos);
                return
            }
            else if let (TextChange::Delete(current_item_range, current_item_text), TextChange::Delete(item_range, change_text)) = (current_item.as_mut(), &change) {
                let start = std::cmp::min(current_item_range.start(), item_range.start());
                let end = std::cmp::max(current_item_range.end(), item_range.end());
                *current_item_range = Range::new(start, end);

                // For now we assume that the text is being deletion is happening without backspace
                current_item_text.push_str(&change_text);
                return  
            }

        }
        self.current_item = Some(change)
    }

    pub fn delete(&mut self, text:String, range: Range) {
        self.replace_current_item(TextChange::Delete(range, text))
    }

    pub fn replace(&mut self, text:String, text_being_replaced: String, range: Range) {
        self.push();
        self.undo_stack.push(TextChange::Replace{range, text, text_being_replaced})
    }

    pub fn push(&mut self) {
        if let Some(value) = self.current_item.take() {
            self.undo_stack.push(value)
        }
    }

    pub fn redo(&mut self) -> Option<TextChange> {
        if let Some(result) = self.redo_stack.pop() {
            let result = result.reverse();
            self.undo_stack.push(result.clone());
            return Some(result)
        }
        None
    }

    pub fn undo(&mut self) -> Option<TextChange> {
        if let Some(result) = self.undo_stack.pop() {
            let result = result.reverse();
            self.redo_stack.push(result.clone());
            return Some(result)
        }
        None
    }
}

struct UndoStack(VecDeque<TextChange>);
impl UndoStack{

    fn new(capacity:usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    fn push(&mut self, change:TextChange) {
        self.0.push_front(change);
    }

    fn pop(&mut self) -> Option<TextChange> {
        self.0.pop_front()
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

#[derive(Clone)]
pub enum TextChange {
    Insert(Position,String),
    Delete(Range,String),
    Replace{
        range: Range,
        text: String, 
        text_being_replaced: String
    }
}

impl TextChange {
    fn reverse(self) -> TextChange {
        match self {
            TextChange::Insert(position,text) => {
                let pos_end = Position::end_of_insert(&text);
                let mut new_end_pos = position.clone();
                TextChange::Delete(Range::new(position, new_end_pos.move_by(pos_end)), text)
            },
            TextChange::Delete(range, text) => {
                TextChange::Insert(range.start(), text)
            } ,
            TextChange::Replace{range, text, text_being_replaced} => {
                TextChange::Replace{range, text_being_replaced: text, text: text_being_replaced}
            },
        }
    }

    fn as_mut(&mut self) -> &mut TextChange {
        self
    }
}
