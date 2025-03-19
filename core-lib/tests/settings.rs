#[cfg(test)]
mod commands_tests {
    use std::rc::Rc;

    use buffer::position::Position;
    use mlua::Lua;
    use pretty_assertions::{assert_eq, assert_ne};
    use core_lib::{editor::Editor, plugin_dir::{PluginConfig, PluginDir}, settings::SettingsBuilder};

    #[test]
    fn should_build_settings() {
        let lua = Rc::new(Lua::new());

        let mut plugin_config = PluginConfig::default();
        plugin_config.supported_editor_version = String::from("0.1.0");
        let plugin = PluginDir::new("alchemy.add_keyboard_command('b', 'redo')".to_string(), plugin_config);
        let settings = SettingsBuilder::new(lua, "0.1.0", "0.1.0")
        .add_plugin(plugin)
        .add_user_script("alchemy:add_keyboard_command('a', 'undo')")
        .build();

        assert!(settings.keyboard_commands.contains_key("a"))
    }
}