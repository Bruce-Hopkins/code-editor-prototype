use std::collections::VecDeque;
use std::result;

use crate::position::{self, Position};
use crate::selection::Range;

#[derive(Default)]
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

    pub fn insert(&mut self, text:String, position: Position) {
        // TODO if text length is greater than 1 than we should just push. Also if the space character is inserted, we push too.
        self.replace_current_item(TextChange::Insert(position, text));
        self.redo_stack.clear();
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
                let end = current_item_range.end().move_by(item_range.end());
                *current_item_range = Range::new(current_item_range.start(), end);

                // For now we assume that the text is being deletion is happening without backspace
                current_item_text.push_str(&change_text);
                return  
            }
        }
        self.push();
        self.current_item = Some(change)
    }

    pub fn delete(&mut self, text:String, range: Range) {
        self.replace_current_item(TextChange::Delete(range, text));
        self.redo_stack.clear();
    }

    pub fn replace(&mut self, text:String, doomed_text: String, range: Range) {
        self.push();
        self.undo_stack.push(TextChange::Replace{range, text, text_being_replaced: doomed_text});
        self.redo_stack.clear();
    }

    pub fn push(&mut self) {
        if let Some(value) = self.current_item.take() {
            self.undo_stack.push(value)
        }
    }

    pub fn redo(&mut self) -> Option<UndoItem> {
        self.push();
        if let Some(result) = self.redo_stack.pop() {
            let result = result.reverse();
            self.undo_stack.push(result.clone());
            return Some(result.into())
        }
        None
    }

    pub fn undo(&mut self) -> Option<UndoItem> {
        self.push();
        if let Some(result) = self.undo_stack.pop() {
            let result = result.reverse();
            self.redo_stack.push(result.clone());
            return Some(result.into())
        }
        None
    }

}
struct UndoStack {
    stack: VecDeque<TextChange>,
    capacity: usize
}

impl Default for UndoStack {
    fn default() -> Self {
        Self { 
            stack: VecDeque::with_capacity(50), 
            capacity: 50 
        }
    }
}
impl UndoStack{

    fn new(capacity:usize) -> Self {
        Self {
            stack: VecDeque::with_capacity(capacity),
            capacity
        }
    }

    fn push(&mut self, change:TextChange) {
        // If the items reach capacity, then we remove the earlier item
        if self.stack.len() == self.capacity {
            self.stack.pop_back();
        }
        self.stack.push_front(change);
    }

    fn pop(&mut self) -> Option<TextChange> {
        self.stack.pop_front()
    }

    fn clear(&mut self) {
        self.stack.clear()
    }
}



#[derive(Clone, Debug)]
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

impl Into<UndoItem> for TextChange {
    fn into(self) -> UndoItem {
        match self {
            TextChange::Insert(position, text) => UndoItem::Insert(position, text),
            TextChange::Delete(range, _) => UndoItem::Delete(range),
            TextChange::Replace { range, text, text_being_replaced: _ } => UndoItem::Replace(range, text),
        }
    }
}

pub enum UndoItem {
    Insert(Position,String),
    Delete(Range),
    Replace(Range, String)
}

#[cfg(test)]
mod undo_stack_test {
    use crate::position::Position;
    use pretty_assertions::{assert_eq, assert_ne};
    use super::{TextChange, Undo, UndoStack};

    #[test]
    fn should_not_insert_over_capacity() {
        let mut stack = UndoStack::new(2);
        stack.push(TextChange::Insert(Position::default(), String::from("1")));
        stack.push(TextChange::Insert(Position::default(), String::from("2")));
        stack.push(TextChange::Insert(Position::default(), String::from("3")));

        assert_eq!(stack.stack.len(), 2);

        let text_change = stack.stack.pop_back().unwrap();
        if let TextChange::Insert(pos, text) = text_change {
            assert_eq!(text, "2")
        }
        else {
            panic!("Not correct text change value");
        }
    }
} 
