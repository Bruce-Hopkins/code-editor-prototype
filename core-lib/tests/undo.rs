#[cfg(test)]
mod undo_tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use core_lib::{position::Position, selection::Range, undo::{TextChange, Undo, UndoItem}};


    #[test]
    fn should_undo() {
        let mut undo = Undo::new(10);
        undo.add(String::from("123"), Position::new(0, 0));
        undo.push();
    
        undo.add(String::from("456"), Position::new(0, 3));
        undo.push();

        if let UndoItem::Delete(range)= undo.undo().unwrap() {
            assert_eq!(range, Range::new(Position::new(0, 3), Position::new(0, 6)));
        }
        else {
            panic!("Incorrect text change type");
        }
    }

    #[test]
    fn should_redo() {

        let mut undo = Undo::new(10);
        undo.add(String::from("123"), Position::new(0, 0));
        undo.push();
    
        undo.add(String::from("456"), Position::new(0, 3));
        undo.push();

        undo.undo();
        
        if let UndoItem::Insert (position, text) = undo.redo().unwrap() {
            assert_eq!(text, String::from("456"));
            assert_eq!(position, Position::new(0, 3));
        }
        else {
            panic!("Incorrect text change type");
        }
        
    }

    #[test]
    fn should_undo_delete() {

        let mut undo = Undo::new(10);
        undo.add(String::from("123"), Position::new(0, 0));
        undo.push();
    
        undo.delete(String::from("123"), Range::new(Position::new(0, 0), Position::new(0,2)));
        undo.push();

        if let UndoItem::Insert (position, text) = undo.undo().unwrap() {
            assert_eq!(text, String::from("123"));
            assert_eq!(position, Position::new(0, 0));
        }
        else {

            panic!("Incorrect text change type");
        }
    }

    #[test]
    fn should_undo_replace() {

        let mut undo = Undo::new(10);
        undo.add(String::from("123"), Position::new(0, 0));
        undo.push();
    
        undo.replace(String::from("456"), String::from("123"), Range::new(Position::new(0, 0), Position::new(0,2)));
        undo.push();

        if let UndoItem::Replace(range, text) = undo.undo().unwrap() {
            assert_eq!(text, String::from("123"));
            assert_eq!(range, Range::new(Position::new(0, 0), Position::new(0,2)));
        }
        else {
            panic!("Incorrect text change type");
        }
        if let UndoItem::Replace(range, text) = undo.redo().unwrap() {
            assert_eq!(text, String::from("456"));
            assert_eq!(range, Range::new(Position::new(0, 0), Position::new(0,2)));
        }
        else {
            panic!("Incorrect text change type");
        }
    }

    #[test]
    fn undo_should_not_store_more_than_capacity() {
        let mut undo = Undo::new(2);
        undo.add(String::from("1"), Position::new(0, 0));
        undo.push();

        undo.add(String::from("2"), Position::new(0, 0));
        undo.push();
        
        undo.add(String::from("3"), Position::new(0, 0));
        undo.push();

    }
}
