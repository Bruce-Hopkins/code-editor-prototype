#[cfg(test)]
mod document_tests {
    use engine::{document::{self, Document}, position::Position};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn document_delete_success() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.delete(
            &Position::new(1, 0), 
            &Position::new(2, 0)
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());

        assert_eq!(Document::from("one\nthree").to_string(), document.to_string());
    }

    #[test]
    fn document_delete_over_range_success() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.delete(
            &Position::new(1, 0), 
            &Position::new(4, 0)
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());

        assert_eq!(Document::from("one\n").to_string(), document.to_string());
    }

    #[test]
    fn document_delete_empty_fail() {
        let mut document = Document::new();
        let result = document.delete(
            &Position::new(1, 0), 
            &Position::new(2, 0)
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }


    #[test]
    fn document_delete_start_line_doesnt_exist_fail() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.delete(
            &Position::new(5, 0), 
            &Position::new(6, 0)
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn document_delete_same_position_fail() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.delete(
            &Position::new(2,2), 
            &Position::new(2, 2)
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn document_delete_start_is_higher_than_end_fail() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.delete(
            &Position::new(1,2), 
            &Position::new(0, 2)
        );
        assert!(result.is_err());
    }
}