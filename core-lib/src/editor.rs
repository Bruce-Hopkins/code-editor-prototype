use std::{fs::{self, File, ReadDir}, io::Read, path::{self, Path, PathBuf}, rc::Rc};

use buffer::{buffer::Buffer, position::Position, selection::SelectionList};
use rustc_hash::FxHashMap;
use mlua::{Function, Lua, Result, Scope, Table, UserData, UserDataFields, UserDataMethods};
use dirs;

use crate::{commands::Commands, plugin_dir::PluginDir, settings::Settings};

fn create_file_if_does_not_exist(path:&Path) -> std::io::Result<File> {
    if !path.is_file() {
        return File::create(path);
    }
    File::open(path)
}

pub struct Editor {
    tabs: Vec<Tab>,
    buffers: FxHashMap<String, Buffer>,
    workspace: Option<PathBuf>,
    active_buffer: Option<Buffer>,
    lua: Rc<Lua>,
    settings: Settings,
    commands: Commands,
}

impl Default for Editor {
    fn default() -> Self {
        Self { 
            tabs: Default::default(), 
            buffers: Default::default(), 
            workspace: Default::default(), 
            active_buffer: Default::default(), 
            lua: Default::default(),
            commands: Default::default(),
            settings: Settings::new(Rc::new(Lua::new()), Self::EDITOR_VERSION, Self::MAJOR_UPDATE_VERSION)
        }
    }
}

fn create_dir_if_does_not_exist(path: &Path) -> std::io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?
    }
    Ok(())
}


pub enum CommandOrFunction {
    Command(String),
    Function(String)
}

impl CommandOrFunction {
    pub fn val(&self) -> &str {
        match self {
            CommandOrFunction::Command(val) => val,
            CommandOrFunction::Function(val) => val,
        }
    }

    pub fn is_command(&self) -> bool {
        if let Self::Command(_) = self {
            return true
        }
        false
    }
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

        let config = editor.settings.add_script(&editor.get_settings_config());
        if let Some(config) = config.as_ref() {
            editor.settings.modify_active_plugins(config);
        }
        editor.load_plugins();
        if let Some(config) = config {
            editor.settings.add_editor_configuration(config);
        }
        editor
    }

    fn load_workspace_script(&mut self, path: &Path) {
        // self.load_script(todo!());
        // self.load_plugins();
        todo!()
    }

    fn get_all_plugins_directories() -> std::io::Result<ReadDir> {
        let mut path = Self::config_dir();
        path.push("plugins");
        create_dir_if_does_not_exist(&path)?;
        
        fs::read_dir(path)
    }

    pub fn config_dir() -> PathBuf {
        let mut config_dir = dirs::config_dir().expect("Could not find config directory");
        config_dir.push("alchemy");
        config_dir
    }

    fn get_settings_config(&self) -> String {
        let mut config_dir = Self::config_dir();
        config_dir.push("config.lua");

        let mut file = create_file_if_does_not_exist(&config_dir).expect("Couldn't create file");
        let mut script = String::new();
        file.read_to_string(&mut script).unwrap();
        script
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

    fn load_plugins(&mut self) {
        let plugin_dirs = Self::get_all_plugins_directories()
        .expect("Failed getting plugin directory");

        for dir in plugin_dirs {
            if let Ok(entry) = dir {
                let path = entry.path();
                let plugin = PluginDir::initialize(path);
                let config = self.settings.add_plugin(plugin);
                if let Some(config) = config {
                    self.settings.add_editor_configuration(config);
                }
            }
        }
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

impl Key {
    pub fn from_str(key: &str) -> Key {
        match key {
            "a" => Key::A,
            "b" => Key::B,
            "c" => Key::C,
            "d" => Key::D,
            "e" => Key::E,
            "f" => Key::F,
            "g" => Key::G,
            "h" => Key::H,
            "i" => Key::I,
            "j" => Key::J,
            "k" => Key::K,
            "l" => Key::L,
            "m" => Key::M,
            "n" => Key::N,
            "o" => Key::O,
            "p" => Key::P,
            "q" => Key::Q,
            "r" => Key::R,
            "s" => Key::S,
            "t" => Key::T,
            "u" => Key::U,
            "v" => Key::V,
            "w" => Key::W,
            "x" => Key::X,
            "y" => Key::Y,
            "z" => Key::Z,
            "f1" => Key::F1,
            "f2" => Key::F2,
            "f3" => Key::F3,
            "f4" => Key::F4,
            "f5" => Key::F5,
            "f6" => Key::F6,
            "f7" => Key::F7,
            "f8" => Key::F8,
            "f9" => Key::F9,
            "f10" => Key::F10,
            "f11" => Key::F11,
            "f12" => Key::F12,
            "arrowup" => Key::ArrowUp,
            "arrowdown" => Key::ArrowDown,
            "arrowleft" => Key::ArrowLeft,
            "arrowright" => Key::ArrowRight,
            "escape" => Key::Escape,
            "enter" => Key::Enter,
            "tab" => Key::Tab,
            "backspace" => Key::Backspace,
            "insert" => Key::Insert,
            "delete" => Key::Delete,
            "pageup" => Key::PageUp,
            "pagedown" => Key::PageDown,
            "home" => Key::Home,
            "end" => Key::End,
            "capslock" => Key::CapsLock,
            "shift" => Key::Shift,
            "ctrl" => Key::Ctrl,
            "alt" => Key::Alt,
            "space" => Key::Space,
            "comma" => Key::Comma,
            "period" => Key::Period,
            "slash" => Key::Slash,
            "backslash" => Key::Backslash,
            "semicolon" => Key::Semicolon,
            "apostrophe" => Key::Apostrophe,
            "leftbracket" => Key::LeftBracket,
            "rightbracket" => Key::RightBracket,
            "minus" => Key::Minus,
            "equal" => Key::Equal,
            "grave" => Key::Grave,
            _ => Key::Unknown,
        }
    }

    pub fn is_valid_key(&self) -> bool {
        if let Self::Unknown = self {
            return false
        }
        true
    }
}

pub struct EditorConfig {
    pub keyboard_commands: FxHashMap<String, CommandOrFunction>,
    pub commands: Commands,
    pub activated_plugins: Vec<String>,
    pub deactivated_plugins: Vec<String>,
    lua: Rc<Lua>
}

impl EditorConfig {
    pub fn new(lua: Rc<Lua>) -> Self {
        EditorConfig {
            keyboard_commands: FxHashMap::default(),
            commands: Commands::default(),
            activated_plugins: vec![],
            deactivated_plugins: vec![],
            lua
        }
    }

    pub fn from_script(lua: Rc<Lua>, script: &str) -> Option<Self> {
        let config = Self::new(lua).load_script(script).unwrap();
        Some(config)
    }

    fn is_valid_keyboard_key(&self, key:&str) -> bool {
        let key = Key::from_str(key);
        key.is_valid_key()
    }

    // TODO Figure out how to handle ctrl and shift
    fn is_valid_command(&self, key: &String) -> bool {
        self.commands.hashmap().contains_key(key.to_lowercase().as_str())
    }

    pub fn load_script(self, script: &str) -> mlua::Result<Self> {
        let mut config = self;
        let lua = config.lua.clone();
        lua.scope(|scope| {
            lua.globals().set("alchemy", scope.create_userdata_ref_mut(&mut config)?)?;
            lua.load(script).exec()?;
            Ok(())
        })?;
        Ok(config)
    }
}

impl UserData for EditorConfig {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("val", |_, this| Ok(5));
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method_mut("add_keyboard_command", |_, this,  value: (String, String)| {
            if !this.is_valid_keyboard_key(&value.0) {
                return Err(mlua::Error::RuntimeError(format!("Failed to add command. Key {} does not exist.", value.0)));
            }
            if !this.is_valid_command(&value.1) {
                return Err(mlua::Error::RuntimeError(format!("Failed to add command. Command {} does not exist.", value.1)));
            }
            this.keyboard_commands.insert(value.0, CommandOrFunction::Command(value.1));
            Ok(())
        });

        methods.add_method_mut("add_custom_keyboard_command", |_, this, value: (String, Function)| {
            if !this.is_valid_keyboard_key(&value.0) {
                return Err(mlua::Error::RuntimeError(format!("Failed to add command. Key {} does not exist.", value.0)));
            }
            let table:Option<Table>  = this.lua.globals().get(value.0.as_str())?;
            let table = match table {
                Some(table) => table,
                None => {
                    // let new_table =  
                    let table = this.lua.create_table()?;
                    this.lua.globals().set(value.0.as_str(), table)?;
                    this.lua.globals().get(value.0.as_str())?
                },
            };
            table.set(value.0.as_str(), value.1)
        });

        methods.add_method_mut("add_activated_plugins", |_, this, value: Vec<String>| {
            this.activated_plugins.extend(value);
            Ok(())
        });

        methods.add_method_mut("add_deactivated_plugins", |_, this, value: Vec<String>| {
            this.deactivated_plugins.extend(value);
            Ok(())
        });

    }
}
