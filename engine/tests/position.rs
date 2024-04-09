#[cfg(test)]
mod color_selector_tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use engine::position::Position;

    #[test]
    fn test_position() {
        let position = Position {line: 4, character:35};
        
        assert_eq!(position, position);
        assert_ne!(position, Position{line: 4, character:34});
        assert_ne!(position, Position{line: 5, character:35});
    }

    #[test]
    fn test_position_gt() {
        let position = Position {line: 4, character:35};
        
        assert!(position > Position{line:3, character:36});
        assert!(position > Position{line: 4, character:10});

        // Should not be greater than
        assert!( !(position > position) );
        assert!( !(position > Position{line:5, character:10}) );
        assert!( !(position > Position{line:4, character:40}) );
    }

    #[test]
    fn test_position_lt() {
        let position = Position {line: 4, character:35};
        
        assert!(position < Position{line:4, character:36});
        assert!(position < Position{line: 5, character:10});

        // Should not be Lesser than
        assert!( !(position < position) );
        assert!( !(position < Position{line:3, character:40}) );
        assert!( !(position < Position{line:4, character:30}) );
    }
}
