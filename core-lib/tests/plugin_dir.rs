#[cfg(test)]
mod plugin_dir {
    use core_lib::{editor::Editor, plugin_dir::PluginDir};

    #[test]
    fn integration_should_open_plugin() {
        let mut test_path = Editor::config_dir();
        test_path.push("test-plugin");
        PluginDir::new(test_path);
    }
}