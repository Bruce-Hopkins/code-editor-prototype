use position::Position;

pub mod position;
pub mod selection;
pub mod document;
pub mod buffer;
pub mod tab;
pub mod diagnostics;

#[derive(Default, Copy, Clone, Debug)]
pub struct Cursor(Position);

impl Cursor {
    pub fn move_horizontally(&mut self, distance: isize) {
        if distance.is_positive() {
            self.0.character = self.0.character.saturating_add(distance as usize);
        } else {
            self.0.character = self.0.character.saturating_sub(distance.abs() as usize);
        }
    }

    pub fn move_vertically(&mut self, distance: isize) {
        if distance.is_positive() {
            self.0.line = self.0.line.saturating_add(distance as usize);
        } else {
            self.0.line = self.0.line.saturating_sub(distance.abs() as usize);
        }
    }

    pub fn move_to_end_of_insert(&mut self, insert: String) {
        let insertion_position = Position::end_of_insert(insert);
        self.0.move_by(insertion_position);
    }
}