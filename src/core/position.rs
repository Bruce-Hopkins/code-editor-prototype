pub trait CursorMessage {
    fn from_cursor_position(pos: Position) -> Self;
    fn from_selection_move(pos: Position) -> Self;
}

#[derive(Debug, Clone, Copy, Default, Eq)]
pub struct Position {
    line: usize,
    character: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            core::cmp::Ordering::Equal => {
                self.character.cmp(&other.character)
            },
            ord => ord,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.character == other.character
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.line.partial_cmp(&other.line) {
            Some(core::cmp::Ordering::Equal) => {
                self.character.partial_cmp(&other.character)
            },
            ord => ord,
        }
        
    }
}

impl From<tree_sitter::Point> for Position {
    fn from(value: tree_sitter::Point) -> Self {
        Self { line: value.row, character: value.column }
    }
}

impl From<lsp_types::Position> for Position {
    fn from(value: lsp_types::Position) -> Self {
        Self { line: value.line as usize, character: value.character as usize }
    }
}

impl Into<lsp_types::Position> for Position {
    fn into(self) -> lsp_types::Position {
        lsp_types::Position{line: self.line as u32, character: self.character as u32}
    }
}

impl Position {

    pub fn new(line: usize, character: usize) -> Self {
        Self {line, character}
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn character(&self) -> usize {
        self.character
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line
    }

    pub fn set_character(&mut self, character: usize) {
        self.character = character
    }
    /**
     * Returns if the pos is greater than or eq the start pos and less than the end pos
     */
    pub fn is_within(&self, start_point: Position, end_point: Position) -> bool {
        self >= &start_point && self < &end_point
    }

    pub fn end_of_insert(content: String) -> Self {
        let lines_vec: Vec<&str> = content.lines().collect();
        let line = lines_vec.len() - 1;
        let character = lines_vec.get(line).unwrap_or(&"").len();
        Position { line, character }
    }

    pub fn add(&mut self, position: Self) {
        let line = self.line + position.line;

        // If it's a new line, set the cursor to be the end of the insert
        let character = if position.line == 0  {
            self.character + position.character
        } else {
            position.character
        };
        *self = Position::new(line, character)
    }
}


#[cfg(test)]
mod color_selector_tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use super::Position;

    #[test]
    fn test_position() {
        let position = Position {line: 4, character:35};
        
        assert_eq!(position, position);
        assert_ne!(position, Position{line: 4, character:34});
        assert_ne!(position, Position{line: 5, character:35});
    }

    #[test]
    fn test_position_gt() {
        let position = Position {line: 4, character:35};
        
        assert!(position > Position{line:3, character:36});
        assert!(position > Position{line: 4, character:10});

        // Should not be greater than
        assert!( !(position > position) );
        assert!( !(position > Position{line:5, character:10}) );
        assert!( !(position > Position{line:4, character:40}) );
    }

    #[test]
    fn test_position_lt() {
        let position = Position {line: 4, character:35};
        
        assert!(position < Position{line:4, character:36});
        assert!(position < Position{line: 5, character:10});

        // Should not be Lesser than
        assert!( !(position < position) );
        assert!( !(position < Position{line:3, character:40}) );
        assert!( !(position < Position{line:4, character:30}) );
    }
}


#[derive(Default, Copy, Clone, Debug)]
pub struct Cursor(pub Position);

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
        self.0.add(insertion_position);
    }
}
