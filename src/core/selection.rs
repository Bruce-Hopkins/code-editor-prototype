use std::mem::swap;

use super::position::{Position, Cursor};

#[derive(Default, Debug, Clone, Copy)]
/**
    A struct to represent the selections within the document.

    Most of the time when this struct is used, the `correct_position` method should be called to sort the values `start` and `end`
 */
pub struct Selection(Range);


impl Selection {

    pub fn start(&self) -> &Position {
        &self.0.start
    }

    pub fn end(&self) -> &Position {
        &self.0.end
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
    pub fn set_start(&mut self, pos: Position) {
        self.0 = Range::new(pos, self.0.end());
    }

    /**
        Sets the end position of the selection. 
     */
    pub fn set_end(&mut self, pos: Position) {
        self.0 = Range::new(self.0.start(), pos);
    }


    /**
     Checks if the position passed is within the bounds of the start and end position
     */
    pub fn is_within(&self, pos: &Position) -> bool {
        pos >= &self.start() && pos < &self.end()
    }

    /**
     Returns a new selection with the positions corrected.
      
     If the start pos is greater than the end, it will swap the values
     */
    pub fn correct_position(&self) -> Self {
        let mut new_selection = *self;
        if new_selection.start() > new_selection.end() {
            swap(&mut new_selection.0.start, &mut new_selection.0.end)
        }
        new_selection
    }

    /**
     * Sets both positions of the selection to the smallest value.
     * 
     * Sets the cursor to the position of the selection
     */
    pub fn clear(&mut self, cursor: &mut Cursor) {
        let position = std::cmp::min(self.0.start, self.0.end);
        self.0.start = position;
        self.0.end = position;
        cursor.0 = position;
    }
}

#[derive(Debug, Clone, Default, Copy)]
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

}

impl From<Selection> for Range {
    fn from(val: Selection) -> Self {
        let selection = val.correct_position();
        selection.0
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