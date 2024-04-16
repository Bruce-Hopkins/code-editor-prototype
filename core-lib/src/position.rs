pub trait CursorMessage {
    fn from_cursor_position(pos: Position) -> Self;
    fn from_selection_move(pos: Position) -> Self;
}

#[derive(Debug, Clone, Copy, Default, Eq)]
pub struct Position {
    pub line: usize,
    pub character: usize,
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

    /**
     * Used to move the position after an insert has been made.
     */
    pub fn move_by(&mut self, position: Self) -> Position {
        let line = self.line + position.line;

        let character = if position.line == 0  {
            // If it is a single line insert.
            self.character + position.character
        } else {
            position.character
        };
        Position::new(line, character)
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
        self.0.move_by(insertion_position);
    }
}
