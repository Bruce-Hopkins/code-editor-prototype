use std::path::PathBuf;

pub struct PluginDir {
    path: PathBuf,
    name: String,
    title: String,
    supported_editor_version: String,
    plugin_version: String,
    active: bool,
    script: String
}

impl PluginDir {
    pub fn new(path: PathBuf) -> Self {
        // Open config file in dir
        todo!()
    }

    pub fn is_active(&self) -> bool {
        self.active
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

