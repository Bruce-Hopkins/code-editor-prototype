#[cfg(test)]
mod position_test {
    use core_lib::position::Position;
    
    #[test]
    fn position_should_be_gt() {
        let position = Position {line: 4, character:35};
        
        assert!(position > Position{line:3, character:36});
        assert!(position > Position{line: 4, character:10});

        // Should not be greater than
        assert!( !(position > position) );
        assert!( !(position > Position{line:5, character:10}) );
        assert!( !(position > Position{line:4, character:40}) );
    }

    #[test]
    fn position_should_be_lt() {
        let position = Position {line: 4, character:35};
        
        assert!(position < Position{line:4, character:36});
        assert!(position < Position{line: 5, character:10});

        // Should not be Lesser than
        assert!( !(position < position) );
        assert!( !(position < Position{line:3, character:40}) );
        assert!( !(position < Position{line:4, character:30}) );
    }
}
