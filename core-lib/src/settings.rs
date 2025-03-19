use std::{fs::{self, ReadDir}, path::{Path, PathBuf}, rc::Rc};

use mlua::Lua;
use rustc_hash::FxHashMap;

use crate::{commands::Commands, editor::{CommandOrFunction, EditorConfig}, plugin_dir::PluginDir};

pub struct SettingsBuilder {
    user_script: Option<EditorConfig>,
    workspace_script: Option<EditorConfig>,
    plugins: Vec<PluginDir>,
    lua: Rc<Lua>,
    editor_version: &'static str,
    major_update_version: &'static str
}

impl SettingsBuilder {
    pub fn new(lua: Rc<Lua>, editor_version: &'static str, major_update_version: &'static str) -> Self {
        Self { 
            lua,
            user_script: None, 
            workspace_script: None, 
            plugins: vec![],
            editor_version,
            major_update_version,
        }
    }
    
    pub fn add_plugin(mut self, plugin:PluginDir) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn add_user_script(mut self, script: &str) -> Self {
        self.user_script = EditorConfig::from_script(self.lua.clone(), script);
        self
    }

    pub fn add_workspace_script(mut self, script: &str) -> Self {
        self.workspace_script = EditorConfig::from_script(self.lua.clone(), script);
        self
    }

    pub fn build(self) -> Settings {
        let mut settings_builder = self;
        let mut active_plugins = vec![];
        let mut deactivated_plugins = vec![];

        if let Some(config) = settings_builder.user_script.as_mut() {
            active_plugins.append(&mut config.activated_plugins);
            deactivated_plugins.append(&mut config.activated_plugins);
        }
        if let Some(config) = settings_builder.workspace_script.as_mut() {
            active_plugins.append(&mut config.activated_plugins);
            deactivated_plugins.append(&mut config.activated_plugins);            
        }
        let mut settings = Settings::new(settings_builder.lua, settings_builder.editor_version, settings_builder.editor_version);
        settings.activated_plugins = active_plugins;
        settings.deactivated_plugins = deactivated_plugins;
        for plugin in settings_builder.plugins {
            settings.add_plugin(plugin);
        }
        if let Some(user_script) = settings_builder.user_script {
            settings.add_editor_configuration(user_script);
        }
        if let Some(workspace_script) = settings_builder.workspace_script {
            settings.add_editor_configuration(workspace_script);
        }
        settings
    }
}

pub struct Settings {
    pub keyboard_commands: FxHashMap<String, CommandOrFunction>,
    pub activated_plugins: Vec<String>,
    pub deactivated_plugins: Vec<String>,
    pub commands: Commands,
    lua: Rc<Lua>,

    editor_version: &'static str,
    major_update_version: &'static str,
}

impl Settings {
    pub fn new(lua: Rc<Lua>, editor_version: &'static str, major_update_version: &'static str) -> Self {
        Self {
            keyboard_commands: FxHashMap::default(),
            activated_plugins: Vec::new(),
            deactivated_plugins: Vec::new(),
            commands: Commands::default(),
            lua,

            editor_version: editor_version,
            major_update_version: major_update_version
        }
    }

    // TODO Change configuation scripts to be theiir own struct
    pub fn add_script(&mut self, script: &String) -> Option<EditorConfig> {
        let config = self.load_script(script).unwrap();
        Some(config)
    }

    /**
     * Modifies the settings to the plugin, as long as the plugin is not `deactivated_plugins` and it is either active or within the `activated_plugins` lists
     */
    pub fn add_plugin(&mut self, plugin: PluginDir) -> Option<EditorConfig> {
        let is_activated_from_settings = self.activated_plugins
            .clone()
            .into_iter()
            .find(|act_p| act_p.as_str() == plugin.name().as_str())
            .is_some();

        let is_deactivated_from_settings = self.deactivated_plugins
            .clone()
            .into_iter()
            .find(|act_p| act_p.as_str() == plugin.name().as_str())
            .is_some();

        if !is_deactivated_from_settings {
            if (plugin.is_active() || is_activated_from_settings) && plugin.is_correct_verison(&self.editor_version, &self.major_update_version) {
                match self.load_script(&plugin.script()) {
                   Err(e) => eprintln!("Failed to load plugin"),
                   Ok(config) => return Some(config),
                }
            }
        }
        None
    }


    pub fn modify_active_plugins(&mut self, config: &EditorConfig) {
        self.activated_plugins.extend(config.activated_plugins.clone());
        self.deactivated_plugins.extend(config.deactivated_plugins.clone());
    }

    pub fn add_editor_configuration(&mut self, config: EditorConfig) {
        if !config.keyboard_commands.is_empty() {
            self.keyboard_commands = config.keyboard_commands;
        }
    }

    fn load_script(&self, script: &str) -> mlua::Result<EditorConfig> {
        EditorConfig::new(self.lua.clone()).load_script(script)
    }


    fn change_activated_plugins(&mut self, activated_plugins: Vec<String>, deactivated_plugins: Vec<String>, )  {
        if !activated_plugins.is_empty() || !deactivated_plugins.is_empty() {
            if !activated_plugins.is_empty() {
                self.activated_plugins = activated_plugins;
            }
            if !deactivated_plugins.is_empty() {
                self.deactivated_plugins = deactivated_plugins;
            }
        }    
    }

}