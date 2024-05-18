#[cfg(test)]
mod commands_tests {
    use buffer::position::Position;
    use pretty_assertions::{assert_eq, assert_ne};
    use core_lib::editor::Editor;

    #[test]
    fn should_goto_passed_position() {
        let mut editor = Editor::default();
        editor.new_file();
        editor.insert("123 456 789");
        editor.goto(Position::new(0, 0));

        let selections = editor
        .get_buffer()
        .as_ref()
        .unwrap()
        .get_selections();

        assert_eq!(selections[0].selection_end(), &Position::new(0, 0));
        assert_eq!(selections[0].selection_start(), &Position::new(0, 0));

    }


    #[test]
    fn should_find_a_unused_filename() {
        let mut editor = Editor::default();
        editor.new_file();
        editor.new_file();
        editor.new_file();

        let buffer_name = editor
        .get_buffer()
        .as_ref()
        .unwrap()
        .get_filename();

        assert_eq!(buffer_name, "untitled-doc-3");
    }
}