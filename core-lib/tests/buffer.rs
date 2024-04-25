#[cfg(test)]
mod buffer_tests {
    use core_lib::{buffer::Buffer, position::Position};
    use highlighter::highlighter::{Highlighter, HighlighterConfig};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn should_insert_at_cursor() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 3));
        buffer.insert(&"\n");

        assert_eq!(&buffer.to_string(), "123\n")
    }

    #[test]
    fn should_delete_at_cursor() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 2));
        buffer.delete();

        assert_eq!(&buffer.to_string(), "12");
    }

    #[test]
    fn should_insert_multiple_character_at_cursor() {
        let mut buffer = Buffer::default();
        buffer.insert(&"1");
        buffer.insert(&"2");
        buffer.insert(&"3");

        assert_eq!(&buffer.to_string(), "123")
    }


    #[test]
    fn should_delete_at_selection() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 0));
        buffer.move_selection(Position::new(0, 3));
        buffer.delete();

        assert_eq!(&buffer.to_string(), "");
    }

    #[test]
    fn should_insert_with_multiple_cursor() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123\n456\n789");
        buffer.move_cursor(Position::new(0, 0));
        buffer.add_cursor(Position::new(1, 0));
        buffer.add_cursor(Position::new(2, 0));
        buffer.insert(&"+");

        assert_eq!(&buffer.to_string(), "+123\n+456\n+789");
    }

    #[test]
    fn should_delete_with_multiple_cursor() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123\n456\n789");
        buffer.move_cursor(Position::new(0, 0));
        buffer.add_cursor(Position::new(1, 0));
        buffer.add_cursor(Position::new(2, 0));
        buffer.delete();

        assert_eq!(&buffer.to_string(), "23\n56\n89");
    }

    #[test]
    fn should_replace_text() {
        let mut buffer = Buffer::default();
        buffer.insert(&"1XX4");
        buffer.move_cursor(Position::new(0, 1));
        buffer.move_selection(Position::new(0, 3));
        buffer.insert(&"23");

        assert_eq!(&buffer.to_string(), "1234")
    }


    #[test]
    fn should_highlight_with_insert() {
        let code = "fn main() {}";
        let mut buffer = Buffer::default();
        let highlighter_config = HighlighterConfig::rust_config(&"");
        buffer.set_highlighter(highlighter_config);

        buffer.insert(&code);
        let mut highlighter = buffer.highlighter().unwrap();
        let highlighter_iter = highlighter.highlighter_iter();
        assert_eq!(highlighter_iter.count(), 6);

        buffer.insert(&"fn secondary() {}");
        let mut highlighter = buffer.highlighter().unwrap();
        assert_eq!(highlighter.highlighter_iter().count(), 12);
    }

    #[test]
    fn should_undo_buffer_insert() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.undo();

        assert_eq!(buffer.to_string(), "");
    }

    #[test]
    fn should_redo_buffer_insert() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.undo();
        buffer.redo();
        assert_eq!(buffer.to_string(), "123");
    }


    #[test]
    fn should_undo_buffer_delete() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 0));
        buffer.move_selection(Position::new(0, 3));

        buffer.delete();
        buffer.undo();

        assert_eq!(buffer.to_string(), "123");
    }


    #[test]
    fn should_redo_buffer_delete() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 0));
        buffer.move_selection(Position::new(0, 3));
        buffer.delete();
        buffer.undo();
        buffer.redo();

        assert_eq!(buffer.to_string(), "");
    }

    #[test]
    fn should_undo_buffer_replace() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 0));
        buffer.move_selection(Position::new(0, 3));
        buffer.insert(&"456");
        buffer.undo();

        assert_eq!(buffer.to_string(), "123");
    }

    #[test]
    fn should_redo_buffer_replace() {
        let mut buffer = Buffer::default();
        buffer.insert(&"123");
        buffer.move_cursor(Position::new(0, 0));
        buffer.move_selection(Position::new(0, 3));
        buffer.insert(&"456");
        buffer.undo();
        buffer.redo();

        assert_eq!(buffer.to_string(), "456");
    }
}
