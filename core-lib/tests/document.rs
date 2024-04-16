#[cfg(test)]
mod document_tests {
    use core_lib::{document::{self, Document}, position::Position};
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

    #[test]
    fn document_insert() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.insert(
            &Position::new(2,5), 
            "\nfour".to_owned()
        );
        assert!(result.is_some());
        assert_eq!(document.to_string(), "one\ntwo\nthree\nfour")
    }

    #[test]
    fn document_insert_fail() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.insert(
            &Position::new(3,6), 
            "\nfour".to_owned()
        );
        assert!(result.is_none());
    }

    #[test]
    fn document_insert_replace() {
        let mut document = Document::from("one\ntwo\nthree");
        let result = document.replace(
            &Position::new(1,0), 
            &Position::new(1,3), 
            "2".to_owned()
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_some());

        assert_eq!(document.to_string(), "one\n2\nthree")
        
    }

}