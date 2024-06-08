use std::{fs::File, io::Read, path::{Path, PathBuf}};

use serde::Deserialize;

#[derive(Deserialize)]

struct PluginConfig {
    name: String,
    title: String,
    #[serde(rename(deserialize = "supported-editor-version"))]
    supported_editor_version: String,

    #[serde(rename(deserialize = "version"))]
    plugin_version: String,
    active: bool,
}

pub struct PluginDir {
    config: PluginConfig,
    path: PathBuf,
    script: String,
}

impl PluginDir {
    pub fn new(path: PathBuf) -> Self {
        // Open config file in dir
        let mut config_path = path.clone();
        config_path.push("config.toml");

        // TODO, properly error handle this
        let mut config_file = File::open(config_path).expect("Could not open config file");

        let mut contents = String::new();
        config_file.read_to_string(&mut contents).expect("Could not read the file to string");

        let mut script = String::new();
        let mut script_path = path.clone();
        script_path.push("main.lua");
        let mut script_file = File::open(script_path).expect("No script found");
        script_file.read_to_string(&mut script).expect("Could not read script");
        
        let config: PluginConfig = toml::from_str(&contents).unwrap();
        Self {
            config,
            path,
            script
        }
    }

    pub fn is_active(&self) -> bool {
        self.config.active
    }

    pub fn is_correct_verison(&self, editor_version:&str, major_update_ver: &str) -> bool {
        todo!()
    }

    pub fn script(&self) -> String {
        todo!()
    }

    pub fn name(&self) -> String {
        todo!()
    }
}

