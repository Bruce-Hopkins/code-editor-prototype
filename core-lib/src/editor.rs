use buffer::{buffer::Buffer, position::Position};
use rustc_hash::FxHashMap;
#[derive(Default)]
pub struct Editor {
    tabs: Vec<Tab>,
    buffers: FxHashMap<String, Buffer>,
    workspace: Option<String>,
    active_buffer: Option<Buffer>
}

impl Editor {
    fn open_directory(&mut self) {
        todo!()
    }

    fn undo(&mut self) {
        todo!()
    }

    fn redo(&mut self) {
        todo!()
    }

    fn add_cursor(&mut self, pos: Position) {
        todo!()
    }

    fn goto(&mut self, file: FileLocation) {
        todo!()
    }
    
    fn run_command(&mut self, command_name: &str) {
        todo!()
    }

    fn find(&mut self) {
        todo!()
    }

    pub fn get_mut_buffer(&mut self) -> &mut Option<Buffer> {
        &mut self.active_buffer
    }
}

struct Tab {
    file_path: String,
    is_active: bool,
}

struct FileLocation {
    position: Position,
    path: String,
}