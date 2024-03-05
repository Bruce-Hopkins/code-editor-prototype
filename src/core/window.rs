use iced::widget::scrollable::AbsoluteOffset;

use super::position::Cursor;

#[derive(Default, Copy, Clone, Debug)]
pub struct VirtualWindow {
    width: f32,
    offset_x: f32,
    offset_y: f32,
    height: f32,
    lineheight: f32,
}

pub enum MoveDirectionY {
    Up,
    Down,
}

pub enum MoveDirectionX {
    Left,
    Right,
}

impl VirtualWindow {

    /* 
        Creates a new window with the default pixel height of 4000px.
    */
    pub fn new() -> Self {
        Self {
            height: 4000.0,
            ..Self::default()
        }
    }

    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
    }

    pub fn size_is_different(&self, width: f32, height: f32)  -> bool{
        self.width == width && height == self.height
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn correct_position_to_cursor(
        &mut self,
        cursor: Cursor,
        text_width: f32,
        longest_line: usize,
        lines: usize,
    ) {
        if cursor.0.line() > self.end_line() {
            // Scroll down
            self.offset_y = self.lineheight * cursor.0.line() as f32 - self.height
        } else if cursor.0.line() < self.start_line() {
            // Scroll up
            self.offset_y = self.lineheight * cursor.0.line() as f32
        }
        if cursor.0.character() > self.end_character(text_width) {
            // Scroll right
            self.offset_x = text_width * cursor.0.character() as f32 - self.width
        } else if cursor.0.character() < self.start_character(text_width) {
            // Scroll left
            self.offset_x = text_width * cursor.0.character() as f32
        }
        self.correct_possition_x(longest_line, text_width);
        self.correct_possition_y(lines);
    }

    fn correct_possition_y(&mut self, lines: usize) {
        if self.offset_y < 0.0 {
            self.offset_y = 0.0
        }
        if self.offset_y > lines as f32 * self.lineheight {
            self.offset_y = lines as f32 * self.lineheight
        }
    }

    fn correct_possition_x(&mut self, longest_line: usize, text_width: f32) {
        if self.offset_x < 0.0 {
            self.offset_x = 0.0
        }
        if self.offset_x > longest_line as f32 * text_width {
            self.offset_x = longest_line as f32 * text_width
        }
    }

    pub fn move_offset_y(&mut self, direction: MoveDirectionY, lines: usize) {
        match direction {
            MoveDirectionY::Up => self.offset_y -= self.lineheight,
            MoveDirectionY::Down => self.offset_y += self.lineheight,
        }
        self.correct_possition_y(lines);
    }

    pub fn move_offset_x(&mut self, direction: MoveDirectionX, longest_line: usize, text_width: f32) {
        match direction {
            MoveDirectionX::Left => self.offset_x -= text_width,
            MoveDirectionX::Right => self.offset_x += text_width,
        }
        self.correct_possition_x(longest_line, text_width);
    }

    /**
     * The padding of the virual window in pixels
     */
    fn padding(self) -> f32 {
        self.lineheight * 2.0
    }
    
    pub fn set_lineheight(self, height: f32) -> Self {
        Self {
            lineheight: height,
            ..self
        }
    }

    pub fn within(&self, y: f32) -> bool {
        let offset = if self.offset_y < self.padding() {
            0.0
        } else {
            self.offset_y - self.padding()
        };

        let height = self.height + self.padding() + self.offset_y;

        y >= offset && y < height
    }

    pub fn start_line(&self) -> usize {
        (self.offset_y / self.lineheight).round() as usize
    }

    pub fn end_line(&self) -> usize {
        (self.offset_y / self.lineheight + self.height / self.lineheight).round() as usize
    }

    pub fn lines_height(&self) -> usize {
        self.end_line().saturating_sub(self.start_line())
    }

    pub fn end_character(&self, text_width: f32) -> usize {
        (self.offset_x / text_width + self.width / text_width).floor() as usize
    }

    pub fn start_character(&self, text_width: f32) -> usize {
        (self.offset_x / text_width).round() as usize
    }

    pub fn padded_start_line(&self) -> usize {
        if self.start_line() < 2 {
            return 0;
        }
        self.start_line() - 2
    }

    pub fn is_same(&self, width: f32, height: f32) -> bool {
        self.width == width && self.height == height
    }

    pub fn padded_end_line(&self) -> usize {
        self.end_line() + 2
    }
}

impl From<VirtualWindow> for AbsoluteOffset {
    fn from(val: VirtualWindow) -> Self {
        AbsoluteOffset {
            x: val.offset_x,
            y: val.offset_y
        }
    }
}

