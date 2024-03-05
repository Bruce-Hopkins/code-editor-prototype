use std::cell::Cell;
use std::vec::IntoIter;

use iced::Command;
use iced::advanced::text;
use iced::advanced::text::Renderer as _;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::Cache;
use iced::widget::canvas::Text;
use iced::widget::text::Shaping;
use iced::widget::Canvas;
use iced::Font;
use iced::Pixels;
use iced::Size;

use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};

use crate::core::buffer::Buffer;
use crate::core::buffer::TextInfo;
use crate::core::document_change::DocumentChange;
use crate::core::position::Cursor;
use crate::core::position::Position;
use crate::highlighter::HighlightItem;

use crate::Message;
use crate::Modifiers;
use crate::VirtualWindow;

use crate::highlighter::color_selector::ColorSelector;

use crate::lsp::response::LspResponse;

use super::floating_text::floating_text_element::FloatingElement;
use super::floating_text::floating_text_element::FloatingText;


pub struct Textbox {
    buffer: Buffer,
    line_height: f32,
    font_size: f32,
    text_cache: Cache,
    font: Font,
    longest_line: Cell<usize>,
    text_width: Cell<f32>,
    floating_element: Option<FloatingElement>,
    is_focused: bool,
}

impl Textbox {
    pub fn new(document: Buffer) -> Self {
        let height = text::LineHeight::default().to_absolute(Pixels(14.0));
        Self {
            text_cache: Cache::new(),
            line_height: height.0,
            font_size: 14.0,
            font: Font::MONOSPACE,
            longest_line: Cell::new(0),
            text_width: Cell::new(0.0),
            buffer: document,
            is_focused: false,
            // floating_element: Some(FloatingText::Diagnostic("Something\n".repeat(20).to_owned()))
            floating_element: None
        }
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus
    }

    pub fn move_selection_with_shift(&mut self, modifier: Modifiers) {
        if modifier.shift {
            self.set_selection_end(self.buffer.cursor.0);
        } else {
            self.set_selection(self.buffer.cursor.0);
        }
    }

    pub fn move_up(&mut self, modifier: Modifiers) {
        self.buffer.move_vertically(-1);
        self.move_selection_with_shift(modifier);
        self.clear_floating_elements();
    }

    pub fn move_down(&mut self, modifier: Modifiers) {
        self.buffer.move_vertically(1);
        self.move_selection_with_shift(modifier);
        self.clear_floating_elements();
    }


    pub fn move_left(&mut self, modifier: Modifiers) {
        self.buffer.move_horizontally(-1, &self.text_info());
        self.move_selection_with_shift(modifier);
        self.clear_floating_elements();
    }

    pub fn move_right(&mut self, modifier: Modifiers) {
        self.buffer.move_horizontally(1, &self.text_info());
        self.move_selection_with_shift(modifier);
        self.clear_floating_elements();
    }

    pub fn move_start(&mut self) {
        self.buffer.cursor.0.set_character(0);
    }

    pub fn move_end(&mut self) {
        let line_len = self.buffer.line_len(self.buffer.cursor.0.line());
        self.buffer.cursor.0.set_character(line_len);
    }

    pub fn page_up(&mut self) {
        self.buffer.move_vertically(-(self.buffer.window.lines_height() as isize));
    }
    
    pub fn page_down(&mut self) {
        self.buffer.move_vertically(self.buffer.window.lines_height() as isize);
    }

    pub fn delete(&mut self) {
        self.buffer.delete();
        self.clear_floating_elements();
    }

    pub fn backspace(&mut self) {
        if self.buffer.cursor.0.character() == 0 && self.buffer.cursor.0.line() == 0 && self.buffer.selection.is_empty() {
            return
        }
        self.buffer.move_previous(1, &self.text_info());
        self.buffer.delete();
        self.clear_floating_elements()
    }

    pub fn new_line(&mut self) {
        self.buffer.get_position();

        self.buffer.insert('\n'.to_string());
        self.buffer.move_next(2, &self.text_info());
        self.clear_floating_elements()
    }

    pub fn save(&mut self, workspace: Option<String>) {
        self.buffer.save(workspace);
    }

    pub fn copy(&mut self, commands: &mut Vec<Command<Message>>) {
        self.buffer.copy(commands);
    }

    pub fn paste(&mut self, commands: &mut Vec<Command<Message>>) {
        self.buffer.paste(commands);
    }

    pub fn cut(&mut self, commands: &mut Vec<Command<Message>>) {
        self.buffer.cut(commands);
    }

    pub fn select_all(&mut self, _command: &mut [Command<Message>]) {
        let line = self.buffer.buffer().len().saturating_sub(1);
        let character = self.buffer.buffer().line_len(line);
        let pos = Position::new(line, character);

        self.buffer.selection.set_start(Position::default());
        self.buffer.selection.set_end(pos);
    }
    
    pub fn floating_element<'a>(&self) -> Option<Element<'a, Message, Renderer>>{
        self.floating_element.as_ref().map(|element| element.view_box.show())
    }

    pub fn is_saved(&self) -> bool {
        self.buffer.is_saved()
    }

    pub fn float_position(&self) -> Position {
        let position = self.floating_element.clone().unwrap().position;
        Position::new(position.line() + 1, position.character())
    }

    pub fn set_floating_message(&mut self) {
        let diagnostic = self.buffer.find_diagnostic(None);
        if let Some(value) = diagnostic {
            self.floating_element = Some(FloatingElement {view_box:FloatingText::Diagnostic(value.message), position:value.range.start()});
        }
    }

    pub fn insert(&mut self, character: String) -> Option<DocumentChange> {
        let _character_len = character.len();
        let mut document_change: Option<DocumentChange> = None;
        if let Some(change) = self.buffer.insert(character) {
            document_change = Some(change)
        }
        self.clear_floating_elements();
        document_change
    }

    pub fn process_lsp_response(&mut self, message: LspResponse) {
        match message {
            LspResponse::Diagnostics(diagnostic) => self.buffer.add_diagnostics(diagnostic),
            LspResponse::Progress => println!("Progress!!"),
            _ => ()
        }
    }

    pub fn file(&self) -> Option<&String> {
        self.buffer.filename()
    }

    pub fn clear_floating_elements(&mut self) {
        self.floating_element = None
    }

    pub fn longest_line(&self) -> usize {
        self.longest_line.get()
    }

    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    pub fn get_font(&self) -> Font {
        self.font
    }

    pub fn text_info(&self) -> TextInfo {
        TextInfo { 
            longest_line: self.longest_line.get(), 
            text_width: self.text_width.get()
        }
    }

    pub fn font(self, font: Font) -> Self {
        Self { font, ..self }
    }

    pub fn text_width(&self) -> f32 {
        self.text_width.get()
    }

    pub fn set_window(&mut self, window: VirtualWindow) -> &mut Self {
        self.buffer.set_window(window);
        self
    }

    pub fn window_height(&self) -> f32 {
        self.buffer.window.height()
    }

    pub fn font_size(self, font_size: f32) -> Self {
        Self { font_size, ..self }
    }

    pub fn height(&self) -> f32 {
        let text_height = self.line_height * self.buffer.len() as f32;
        if text_height >= 1980.0 {
            text_height
        } else {
            1980.0
        }
    
    }

    pub fn view(&self) -> Element<Message, Renderer> {
        Canvas::new(self).width(Length::Fill).height(self.height()).into()
    }

    pub fn clear(&self) {
        self.text_cache.clear()
    }

    pub fn set_curor(&mut self, cursor: Position) -> &mut Self {
        self.buffer.set_cursor(Cursor(cursor));
        self
    }

    pub fn set_offset(&mut self, offset_x: f32, offset_y: f32) {
        self.buffer.window.set_offset(offset_x, offset_y)
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn correct_position(&mut self)  {
        self.buffer.correct_position()
    }

    pub fn correct_position_to_cursor(&mut self, text_width: f32, longest_line: usize) {
        self.buffer.correct_position_to_cursor(text_width,longest_line)
    }

    pub fn set_size(&mut self, height: f32, width: f32) {
        self.buffer.window.set_size(width, height)
    }

    pub fn set_selection(&mut self, pos: Position) -> &mut Self {
        self.buffer.selection.set_end(pos);
        self.buffer.selection.set_start(pos);
        self
    }

    pub fn set_selection_end(&mut self, pos: Position) -> &mut Self {
        self.buffer.selection.set_end(pos);
        self
    }

    pub fn get_window_point_from_position(&self, position: Position) -> Point{
        let x = position.character() as f32 * self.text_width.get();
        let y = position.line() as f32 * self.line_height;
        Point { x, y }
    }

    fn draw_line(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        line_number: usize,
        text_width: f32,
        content: &str,
        highlighter: &mut IntoIter<HighlightItem>,
        highlight_item: &mut Option<HighlightItem>,
    ) {
        let y = self.line_height * line_number as f32;
        if !self.buffer.window.within(y) {
            return;
        }

        if self.buffer.cursor.0.line() == line_number && self.buffer.cursor.0.character() == content.len() {
            let point = Point::new(text_width * content.len() as f32, y);
            frame.fill_rectangle(
                point,
                // Size::new(text_width, self.height),
                Size::new(2.0, self.line_height),
                Color::from_rgba(83.0, 83.0, 83.0, 0.2),
            )
        }
        for (c_index, c) in content.chars().enumerate() {
            let x = text_width * c_index as f32;
            let point = Point::new(x, y);

            // Draw Cursor
            if self.buffer.cursor.0.line() == line_number && self.buffer.cursor.0.character() == c_index {
                frame.fill_rectangle(
                    point,
                    // Size::new(text_width, self.height),
                    Size::new(2.0, self.line_height),
                    Color::from_rgba(83.0, 83.0, 83.0, 0.2),
                )
            }

            // Draw selection
            let pos = Position::new( 
                line_number,
                c_index,
            );
            if self.buffer.selection.correct_position().is_within(&pos) && !self.buffer.selection.is_empty() {
                frame.fill_rectangle(
                    point,
                    Size::new(text_width, self.line_height),
                    Color::from_rgba(83.0, 83.0, 83.0, 0.1),
                )
            }

            // Draw error lines
            if let Some(issue) = self.buffer.diagnostic_are_in_position(pos) {
                frame.fill_rectangle(
                    Point { x: point.x, y: point.y + self.line_height - 4.0 },
                    Size::new(text_width, 3.0),
                    issue.severity.color(),
                )
            }
            
            let color_selector = ColorSelector;

            let text = Text {
                position: point,
                font: self.font,
                content: String::from(c),
                size: self.font_size,

                // Check if the highlighted item is in the range of the current function..
                // If so, set the color of the text to the color.
                // If not, get the next value and set the current item to another value
                color: color_selector.select(highlighter, highlight_item, &pos),
                ..Text::default()
            };

            frame.fill_text(text);
        }
    }
}


impl<Message> canvas::Program<Message, Renderer> for Textbox
where
    Renderer: iced::advanced::text::Renderer,
{
    type State = ();

    fn mouse_interaction(
        &self,
        _interaction: &(),
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        let mouse_is_over = cursor.is_over(bounds);
        if mouse_is_over {
            return mouse::Interaction::Pointer;
        }
        mouse::Interaction::default()
    }


    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<<Renderer as canvas::Renderer>::Geometry> {
        let result = self.text_cache.draw(renderer, bounds.size(), |frame| {
            let mut longest_line = 0;
            let mut highlighter = self.buffer.get_highlighter().clone().unwrap().captures.into_iter();
            let mut highlight_item = highlighter.next();
            for (index, line) in self.buffer.lines().enumerate() {
                let width = if self.text_width.get() == 0.0 {
                    let width = renderer.measure_width("T", self.font_size, Font::MONOSPACE, Shaping::Basic);
                    self.text_width.set(width);
                    width
                } else {
                    self.text_width.get()
                };

                if line.len_chars() > longest_line {
                    longest_line = line.len_chars()
                }
                self.draw_line(
                    frame,
                    index,
                    width,
                    &line.to_string(),
                    &mut highlighter,
                    &mut highlight_item,
                )
            }
            self.longest_line.set(longest_line);
        });
        vec![result]
    }
}
