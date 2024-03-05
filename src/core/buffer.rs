

use iced::{widget::text, clipboard, Command};
use ropey::iter::Lines;

use crate::{highlighter::{HighlighterConfig, Highlighter}, Message, lsp::response::{ClientDiagnostics, Issue}};

use super::{document::{ByteRange, Document}, document_change::DocumentChange, position::{Cursor, Position}, selection::{Range, Selection}, window::{MoveDirectionX, MoveDirectionY, VirtualWindow}};


pub struct Buffer {
    document: Document,
    highlighter: HighlighterConfig,
    pub selection: Selection,
    pub cursor: Cursor,
    pub window: VirtualWindow,
    diagnostics: Option<ClientDiagnostics>
}

impl Buffer {

    pub fn new(buffer: Document, tree: HighlighterConfig) -> Self {
        let height = text::LineHeight::default().to_absolute(iced::Pixels(14.0));
        Self { 
            document: buffer, 
            highlighter: tree, 
            cursor: Cursor::default(), 
            window: VirtualWindow::new().set_lineheight(height.0),
            selection: Selection::default(),
            diagnostics: None
        }
    }

    pub fn diagnostic_are_in_position(&self, pos: Position) -> Option<Issue>{
        if let Some(diagnostic) = &self.diagnostics {
            return diagnostic.diagnostic_in_position(pos)
        };
        None
    }

    /**
     * Adds the diagnostic to the buffer if the paths match.
     */
    pub fn add_diagnostics(&mut self, diagonostic: ClientDiagnostics) {
        if let Some(uri) = self.document.uri() {
            if uri == &diagonostic.uri {
                self.diagnostics = Some(diagonostic)
            }
        }
    }

    /**
     * Searches for an issues that is within the range of the positions
     * 
     * if no position is provided it will default to the cursor position
     */
    pub fn find_diagnostic(&self, pos: Option<Position>) -> Option<Issue> {
        let pos = pos.unwrap_or(self.cursor.0);
        if let Some(diagnostics) = &self.diagnostics {
            let issue = diagnostics
                .clone()
                .issues
                .into_iter()
                .find(|issue| {
                    issue.range.pos_in_range(pos)
                });
            return issue
        }
        None
    }

    pub fn filename(&self) -> Option<&String> {
        self.document.filename()
    }

    pub fn save(&mut self, workspace: Option<String>) {
        self.document.save(workspace).unwrap();
    }

    pub fn buffer(&self) -> &Document {
        &self.document
    }

    pub fn set_window(&mut self, window: VirtualWindow) {
        self.window = window;
    }

    pub fn len(&self) -> usize {
        self.document.len()
    }
    
    pub fn is_saved(&self) -> bool {
        self.document.is_saved()
    }

    /**
     * Get's the length of characters within the line
     */
    pub fn line_len(&self, line: usize) -> usize {
        self.document.line_len(line)
    }

    /** 
     * Changes the cursor position of the buffer
     */
    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor
    }

    pub fn get_highlighter(&self) -> Option<Highlighter> {
        let start_line = self.window.padded_start_line();
        let start_line = self.document.get_line_bytes(start_line);

        let end_line = self.window.padded_end_line();
        let end_line = self.document.get_line_bytes(end_line);

        Highlighter::new(&self.highlighter, start_line..end_line, self.buffer())
    }

    pub fn lines(&self)  -> Lines<'_>{
        self.document.lines()
    }

    pub fn replace(&mut self, start_idx:&Position, end_idx:&Position, character: String) {
        self.document.replace(start_idx, end_idx, character);
    }

    pub fn delete(&mut self) {
        let cursor_pos = self.get_position();

        // If the selection is available
        if !self.selection.is_empty() {
            let selection = self.selection.correct_position();
            self.selection.clear(&mut self.cursor);
            let bytes = self.document.delete(selection.start(), selection.end());
            if let Some(bytes) = bytes {
                self.character_deleted(bytes.start, bytes.end, *selection.start(), *selection.end())
            }
        } 
        
        else {
            let start = self.document.delete(&self.cursor.0, &Position::new(self.cursor.0.line(), self.cursor.0.character() + 1));
            if let Some(value) = &start {
                self.character_deleted(value.start, value.end, cursor_pos, cursor_pos)
            }
        }
    }

    fn character_deleted(&mut self, start_byte: usize, end_byte: usize, start_pos: Position, end_pos: Position) {
        self.highlighter.delete(start_byte, end_byte, start_pos, end_pos, &self.document.slice_all())
    }


    fn character_inputed(&mut self, start_byte: usize, end_byte:usize, start_pos: Position, end_pos:Position, length: usize) {
        self.highlighter.insert(start_byte, end_byte, start_pos, end_pos, length, &self.document.slice_all())
    }

    pub fn insert(&mut self, content: String) -> Option<DocumentChange> {
        let cursor_pos = self.get_position();
        // If the selection is available
        if !self.selection.is_empty() {
            let selection = self.selection.correct_position();
            self.selection.clear(&mut self.cursor);

            let string_len = content.len();
            let bytes = self.document.replace(selection.start(), selection.end(), content.clone());
            if let Some(bytes) = bytes {
                self.character_inputed(bytes.start, bytes.end, *selection.start(), *selection.end(), string_len);
                self.cursor.move_to_end_of_insert(content.clone());
                if let Some(filename) = self.document.filename() {
                    return Some(DocumentChange::new(selection.into(), bytes, content, filename.clone()));
                }
            }
            None
        } 
        
        // Input from the cursor positions
        else {
            let string_len = content.len();
            let start = self.document.insert(&self.cursor.0, content.clone());
            self.character_inputed(start, start, cursor_pos, cursor_pos, string_len);
            self.cursor.move_to_end_of_insert(content.clone());
            if let Some(filename) = self.document.filename() {
                return Some(DocumentChange::new(
                    Range::new(cursor_pos,cursor_pos), 
                    ByteRange{start, end: start}, 
                    content,
                    filename.clone()
                ))
            }
            None
        }

        
    }

    fn get_selected_text(&self) -> String {
        let selection = self.selection.correct_position();
        let start_bytes = self.document.get_line_bytes(selection.start().line()) + selection.start().character();
        let end_bytes = self.document.get_line_bytes(selection.end().line()) + selection.end().character();

        let slice = self.document.str_from_range(start_bytes, end_bytes);
        slice.to_string()
    }

    /**
     * Copies the selected text
     */
    pub fn copy(&self, commands: &mut Vec<Command<Message>>){
        let text = self.get_selected_text();
        commands.push(
            clipboard::write::<Message>(text)
        )
    }

    /**
     * Cuts the selected text
     */
    pub fn cut(&mut self, commands: &mut Vec<Command<Message>>){
        let text = self.get_selected_text();
        self.delete();
        
        commands.push(clipboard::write::<Message>(text))
    }

    pub fn paste(&mut self, commands: &mut Vec<Command<Message>>) {
        if !self.selection.is_empty() {
            self.delete();
        };
        let command = clipboard::read(|value| {
            Message::Paste(value.unwrap_or("".to_owned()))
        });
        commands.push(command)
    }

    pub fn get_position(&self) -> Position {
        self.cursor.0
    }

    pub fn correct_position_to_cursor(&mut self, text_width: f32, longest_line: usize) {
        self.window.correct_position_to_cursor(self.cursor, text_width, longest_line, self.len())
    }

    pub fn get_string(&self) -> String {
        self.document.to_string()
    }

    pub fn move_horizontally(&mut self, distance: isize, text_info: &TextInfo) {
        let text_width = text_info.text_width;
        let longest_line = text_info.longest_line;
        self.cursor.move_horizontally(distance);
        self.correct_position();
        if self.cursor.0.character() >= self.window.end_character(text_width) - 5 && distance > 0 {
            self.window
                .move_offset_x(MoveDirectionX::Right, longest_line, text_width)
        }
        if self.cursor.0.character() <= self.window.start_character(text_width) + 5 && distance < 0 {
            self.window
                .move_offset_x(MoveDirectionX::Left, longest_line, text_width)
        }
    }

    pub fn move_vertically(&mut self, distance: isize) {
        if self.cursor.0.line() >= self.window.end_line() - 2 && distance > 0 {
            self.window
                .move_offset_y(MoveDirectionY::Down, self.len())
        }
        if self.cursor.0.line() <= self.window.start_line() + 2 && distance < 0 {
            self.window
                .move_offset_y(MoveDirectionY::Up, self.len())
        }
        self.cursor.move_vertically(distance);
        self.correct_position();
    }

    pub fn get_cursor_row_len(&self) -> usize {
        
        self.line_len(self.cursor.0.line())
    }

    pub fn move_next(&mut self, distance: isize, text_info: &TextInfo) {
        if self.cursor.0.character() + distance as usize > self.get_cursor_row_len() {
            self.move_vertically(1);
            self.cursor.0.set_character(0);
            return;
        }
        self.move_horizontally(distance, text_info);
    }

    pub fn move_previous(&mut self, distance: isize, text_info: &TextInfo) {
        if self.cursor.0.character() as isize - distance < 0 {
            self.move_vertically(-1);
            self.cursor.0.set_character(self.get_cursor_row_len() - 1);

            return;
        }
        self.move_horizontally(-distance, text_info);
    }

    /**
     * Corrects the position of the cursor when it's out of bounds
     */
    pub fn correct_position(&mut self) {
        // This if statement prevents this from crashing
        if self.len() != 0 && self.cursor.0.line() > self.len() - 1 {
            self.cursor.0.set_line(self.len() - 1);
        }
        if self.line_len(self.cursor.0.line()) != 0 {
            let line_length = if self.len() - 1 == self.cursor.0.line() {
                // Let the user have the cursor on the last character if it's the last line.
                self.line_len(self.cursor.0.line())
            } else {
                self.line_len(self.cursor.0.line()) - 1
            };
            if self.cursor.0.character() > line_length {
                self.cursor.0.set_character(line_length);
            }
        }
        else {
            self.cursor.0.set_character(0);
        }
    }

    pub fn is_within_selection(&self, pos: &Position) -> bool {
        self.selection.is_within(pos)
    }
}

pub struct TextInfo {
    pub longest_line: usize,
    pub text_width: f32
}
