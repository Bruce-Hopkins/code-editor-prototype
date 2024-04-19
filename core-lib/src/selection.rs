use std::mem::swap;

use super::position::{Position};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
/**
    A struct to represent the selections within the document.

    Most of the time when this struct is used, the `correct_position` method should be called to sort the values `start` and `end`
 */
pub struct Selection(Range);


impl Selection {

    pub fn new(pos: Position) -> Self {
        Selection(Range::new(pos, pos))
    }

    /**
     * Get's the end of range, which will represent the cursor
     */
    pub fn end(&self) -> Position {
        self.0.end
    }

    fn set_character(&mut self, value: usize) {
        self.0.start.character = value;
        self.0.end.character = value;
    }

    fn set_line(&mut self, value: usize) {
        self.0.start.line = value;
        self.0.end.line = value;
    }

    pub fn move_cursor_horizontally(&mut self, distance: isize) {
        if distance.is_positive() {
            self.set_character(self.0.end.character.saturating_add(distance as usize));
        } else {
            self.set_character(self.0.end.character.saturating_sub(distance.abs() as usize));
        }
    }

    pub fn move_cursor_vertically(&mut self, distance: isize) {
        if distance.is_positive() {
            self.set_line(self.0.end.line.saturating_add(distance as usize));
        } else {
            self.set_line(self.0.end.line.saturating_sub(distance.abs() as usize))
        }
    }

    pub fn move_cursor_to_end_of_insert(&mut self, insert: &str) {
        let insertion_position = Position::end_of_insert(insert);
        self.0.start = self.0.start.move_by(insertion_position);
        self.0.end = self.0.end.move_by(insertion_position);

    }

    pub fn selection_start(&self) -> &Position {
        &self.0.start
    }

    /*
        Returns true if the start and end values are not the same 
    */
    pub fn is_empty(&self) -> bool {
        self.0.start == self.0.end
    }
    
    /**
        Sets the start positions of the selection.
     */
    pub fn move_selection(&mut self, pos: Position) {
        self.0.start = dbg!(pos);
    }

    /**
        Sets the end position of the selection. 
     */
    pub fn set_cursor(&mut self, pos: Position) {
        self.0.end = pos;
        self.0.start = pos;
    }


    /**
     Checks if the position passed is within the bounds of the start and end position
     */
    pub fn is_within(&self, pos: &Position) -> bool {
        pos >= &self.selection_start() && pos < &self.end()
    }

    /**
     Returns a new selection with the positions corrected.
      
     If the start pos is greater than the end, it will swap the values
     */
    pub fn range(&self) -> Range {
        let mut new_range = self.0;
        if &new_range.start() > &new_range.end() {
            swap(&mut new_range.start, &mut new_range.end)
        }
        new_range
    }

    /**
     * Sets both positions of the selection to the smallest value.
     * 
     * Sets the cursor to the position of the selection
     */
    pub fn clear(&mut self) {
        let position = std::cmp::min(self.0.start, self.0.end);
        self.0.start = position;
        self.0.end = position;
    }

    
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq)]
pub struct Range {
    start: Position,
    end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn pos_in_range(&self, pos: Position) -> bool {
        pos >= self.start  &&  pos < self.end
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    /**
     * Returns true if the start and end values in the range are the same
     */
    pub fn is_same(&self) -> bool {
        self.start == self.end
    }

}

impl From<lsp_types::Range> for Range {
    fn from(value: lsp_types::Range) -> Self {
        Self { start: Position::from(value.start), end: Position::from(value.end) }
    }
}

impl Into<lsp_types::Range> for Range {
    fn into(self) -> lsp_types::Range {
        lsp_types::Range {
            start: self.start.into(),
            end: self.end.into()
        }
    }
}
// TODO, can this be a binary heap instead?
#[derive(Clone, Debug)]

pub struct SelectionList(Vec<Selection>);

impl Default for SelectionList { 
    fn default() -> Self {
        Self(vec![Selection::default()])
    }
}

impl SelectionList {

        /**
     * Maintains a sorted list of cursor and adds a new cursor into the mix.
     */
    pub fn push(&mut self, new_cursor: Position) {
        self.0.push(Selection::new(new_cursor));
        self.0.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    }

    /**
     * Sorts the selections in largest to smallest and returns the result.
     */
    pub fn get_all(&self) -> Vec<Selection> {
        let mut selections = self.0.clone(); 
        selections.sort_by(|a, b| b.0.start.cmp(&a.0.start));
        selections
    }

    /**
     * Moves the selection of the last created cursor 
     */
    pub fn move_selection(&mut self, pos: Position) {
        // The selection list should always have one element inside of it
        let last_selection_pos = self.0.len().saturating_sub(1);
        self.0[last_selection_pos].move_selection(pos);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Selection> {
        self.0.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.0.truncate(1);
    }

    pub fn replace_cursor(&mut self, pos: Position) {
        self.0.truncate(0);
        self.push(pos);
    }

}