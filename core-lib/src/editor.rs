use buffer::{buffer::Buffer, position::Position, selection::SelectionList};
use rustc_hash::FxHashMap;

use crate::commands::Commands;
#[derive(Default)]
pub struct Editor {
    tabs: Vec<Tab>,
    buffers: FxHashMap<String, Buffer>,
    workspace: Option<String>,
    active_buffer: Option<Buffer>,
    commands: Commands,
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

    /**
     * Moves the cursor of the active buffer to the passed position
     */
    pub fn goto(&mut self, position: Position) {
        if let Some(buffer) = self.active_buffer.as_mut() {
            let mut selections = SelectionList::default();
            selections.add_cursor(position);
            buffer.set_selections(selections)
        }
    }

    fn goto_file(&mut self, file: FileLocation) {
        todo!()
    }
    
    pub fn run_command(&mut self, command_name: &str) {
        let command = self.commands.get(command_name);
        if let Some(command) = command {
            return command(self);
        }
        eprintln!("Command passed does not exist");
    }

    fn find(&mut self) {
        todo!()
    }

    /**
     * Inserts the passed value into the active buffer.
     */
    pub fn insert(&mut self, value: &'static str) {
        if let Some(buffer) = self.active_buffer.as_mut() {
            buffer.insert(&value);
        }
    }

    /**
     * Changes the active buffer and saves the previous active buffer
     */
    pub fn set_buffer(&mut self, buffer: Buffer) {
        if let Some(buffer) = self.active_buffer.take() {
            self.buffers.insert(dbg!(buffer.get_filename().to_string()), buffer);
        }
        self.active_buffer = Some(buffer)
    }

    /** 
     * It should create a new buffer and a tab. 
     * */
    fn open_file() {
        todo!()
    }

    /**
     * Creates a new buffer. And gives the title of the buffer, `untitled-` plus it's temporary document number.
     * Sets the `active_buffer` to the newly opened buffer.
     */
    pub fn new_file(&mut self) {
        let filename = self.find_filename();
        if let Some(filename) = filename {
            let buffer: Buffer = Buffer::new(&filename);
            self.set_buffer(buffer);
            return
        }
        panic!("There's too many untitled files")
    }

    pub fn get_mut_buffer(&mut self) -> &mut Option<Buffer> {
        &mut self.active_buffer
    }

    pub fn get_buffer(&self) -> &Option<Buffer> {
        &self.active_buffer
    }

    /**
     * Finds an unused named for the new file.
     */
    fn find_filename(&self) -> Option<String> {
        for i in 1..100000 {
            let buffer_name = format!("untitled-doc-{}", i);
            if let None = self.buffers.get(&buffer_name) {
                if let Some(buffer) = &self.active_buffer {
                    if buffer.get_filename() == buffer_name {
                        continue;
                    }
                }
                return Some(buffer_name)
            }
        }
        None
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