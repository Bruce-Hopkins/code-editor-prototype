#[cfg(test)]
mod commands_tests {
    use buffer::position::Position;
    use pretty_assertions::{assert_eq, assert_ne};
    use core_lib::editor::Editor;

    #[test]
    fn should_move_next_word() {
        let mut editor = Editor::default();
        editor.new_file();
        editor.insert("123 456 789");
        editor.goto(Position::new(0, 0));     
        editor.run_command("next_word_end");

        let selections = editor
        .get_buffer()
        .as_ref()
        .unwrap()
        .get_selection();

        assert_eq!(selections[0].selection_end(), &Position::new(0, 3));
        assert_eq!(selections[0].selection_start(), &Position::new(0, 3));
    }

}