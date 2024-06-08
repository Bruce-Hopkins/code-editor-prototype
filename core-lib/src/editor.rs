use std::{fs::{self, File, ReadDir}, io::Read, path::{self, Path, PathBuf}};

use buffer::{buffer::Buffer, position::Position, selection::SelectionList};
use rustc_hash::FxHashMap;
use mlua::{Function, Lua, Result, Table, UserData, UserDataFields};
use dirs;

use crate::{commands::Commands, plugin_dir::PluginDir};

fn create_dir_if_does_not_exist(path: &Path) -> std::io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?
    }
    Ok(())
}

fn create_file_if_does_not_exist(path:&Path) -> std::io::Result<File> {
    if !path.is_file() {
        return File::create(path);
    }
    File::open(path)
}

#[derive(Default)]
pub struct Editor {
    tabs: Vec<Tab>,
    buffers: FxHashMap<String, Buffer>,
    workspace: Option<PathBuf>,
    active_buffer: Option<Buffer>,
    commands: Commands,
    lua: Lua
}

impl Editor {
    const EDITOR_VERSION: &'static str = "0.1.0";

    /**
     * If a plugin uses a editor version that is older than the makor update version, there should be a warning that the plugin might not function correctly
     */
    const MAJOR_UPDATE_VERSION: &'static str = "0.1.0";

    fn open_directory(&mut self, path: PathBuf) {
        if path.is_dir() {
            self.tabs = vec![];
            self.buffers = FxHashMap::default();
            self.active_buffer = None;

            self.load_workspace_script(&path);
            self.workspace = Some(path)
        }
    }

    /**
     * Starts returns the editor while loading the plugins.
     */
    fn start() -> Self {
        let mut editor = Self::default();
        let config_dir = Self::config_dir();
        create_dir_if_does_not_exist(&config_dir).expect("Could not create config dir");

        editor.load_plugins();
        editor.load_settings();
        editor
    }

    pub fn config_dir() -> PathBuf {
        let mut config_dir = dirs::config_dir().expect("Could not find config directory");
        config_dir.push("alchemy");
        config_dir
    }

    fn load_workspace_script(&mut self, path: &Path) {
        self.load_script(todo!(), false);
        todo!()
    }

    fn get_all_plugins_directories() -> std::io::Result<ReadDir> {
        let mut path = Self::config_dir();
        path.push("plugins");
        create_dir_if_does_not_exist(&path)?;
        
        fs::read_dir(path)
    }

    fn load_plugins(&mut self) {
        let plugin_dirs = Self::get_all_plugins_directories()
        .expect("Failed getting plugin directory");

        for dir in plugin_dirs {
            if let Ok(entry) = dir {
                let path = entry.path();
                let plugin_dir = PluginDir::new(path);
                if plugin_dir.is_active() && plugin_dir.is_correct_verison(&Self::EDITOR_VERSION, Self::MAJOR_UPDATE_VERSION) {
                    self.load_script(&plugin_dir.script(), true);
                }
            }
        }
        todo!()
    }

    fn load_settings(&mut self) {
        let mut config_dir = Self::config_dir();
        config_dir.push("config.lua");

        let mut file = create_file_if_does_not_exist(&config_dir).expect("Couldn't create file");
        let mut script = String::new();
        file.read_to_string(&mut script).unwrap();

        // let config_file = include_str!(config_dir);
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

    fn load_script(&mut self, script: &str, is_plugin: bool) {
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

    pub fn key_press(&mut self, key: Key) {
        // Get the command that matches the keypress
        // Run the command
        todo!()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Alphanumeric keys
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Arrow keys
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,

    // Control keys
    Escape, Enter, Tab, Backspace, Insert, Delete,
    PageUp, PageDown, Home, End,
    CapsLock,

    // Modifier keys
    Shift, Ctrl, Alt,

    // Special character keys
    Space, Comma, Period, Slash, Backslash, Semicolon, Apostrophe,
    LeftBracket, RightBracket, Minus, Equal, Grave,

    // Other keys
    Unknown, // To represent an unknown or unsupported key
}