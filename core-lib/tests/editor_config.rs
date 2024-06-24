#[cfg(test)]
mod commands_tests {
    use std::{process::Command, rc::Rc};

    use pretty_assertions::{assert_eq, assert_ne};
    use core_lib::editor::{CommandOrFunction, EditorConfig};

    #[test]
    fn should_set_command_to_key() {
        let lua = mlua::Lua::new();
        let lua = Rc::new(lua);
        let mut config = EditorConfig::new(lua.clone());
        lua.scope(|scope| {
            lua.globals().set("alchemy", scope.create_userdata_ref_mut(&mut config)?)?;
            lua.load("print(alchemy:add_keyboard_command('a', 'undo'))").exec()?;
            Ok(())
        }).unwrap();

        assert!(config.keyboard_commands.contains_key("a"));

        let key_command = config.keyboard_commands.get("a").unwrap();
        assert!(key_command.is_command());
        assert_eq!(key_command.val(), "undo");
    }

}