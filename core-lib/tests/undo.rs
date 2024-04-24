#[cfg(test)]
mod undo_tests {

    use core_lib::{undo::{Undo, TextChange}, position::Position, selection::Range};


    #[test]
    fn should_undo() {
        let mut undo = Undo::new(10);
        undo.add(String::from("123"), Position::new(0, 0));
        undo.push();
    
        undo.add(String::from("456"), Position::new(0, 3));
        undo.push();

        if let TextChange::Delete(range, text)= undo.undo().unwrap() {
            assert_eq!(text, String::from("456"));
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
        
        if let TextChange::Insert (position, text) = undo.redo().unwrap() {
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

        if let TextChange::Insert (position, text) = undo.undo().unwrap() {
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

        if let TextChange::Replace{range, text, text_being_replaced} = undo.undo().unwrap() {
            assert_eq!(text, String::from("123"));
            assert_eq!(range, Range::new(Position::new(0, 0), Position::new(0,2)));
        }
        else {
            panic!("Incorrect text change type");
        }
        if let TextChange::Replace{range, text, text_being_replaced} = undo.redo().unwrap() {
            assert_eq!(text, String::from("456"));
            assert_eq!(range, Range::new(Position::new(0, 0), Position::new(0,2)));
        }
        else {
            panic!("Incorrect text change type");
        }
    }

}
