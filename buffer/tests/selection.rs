#[cfg(test)]
mod selection_test {
    use buffer::{position::Position, selection::{Range, Selection, SelectionList}};
    use pretty_assertions::{assert_eq, assert_ne};
    
    #[test]
    fn should_show_sorted_selection_range() {
        let mut selection = Selection::new(Position::new(2, 0));
        selection.move_selection(Position::new(1, 0));
        assert_eq!(selection.range(), Range::new(Position::new(1, 0), Position::new(2, 0)))
    }

    #[test]
    fn should_move_to_end_of_insert() {
        let insert = "123";
        let mut selection = Selection::default();
        selection.move_cursor_to_end_of_insert(&insert);
        assert_eq!(selection, Selection::new(Position::new(0, 3)));
        
        let insert = "123\n456";
        let mut selection = Selection::default();
        selection.move_cursor_to_end_of_insert(insert);
        assert_eq!(selection, Selection::new(Position::new(1, 3)));
    }

    #[test]
    fn get_all_selections_should_be_sorted() {
        let mut list = SelectionList::default();
        let pos1 = Position::new(1, 0);
        let pos2 = Position::new(2, 0);

        list.push(pos1);
        list.push(pos2);

        assert_eq!(list.get_all(), vec![
            Selection::new(pos2), 
            Selection::new(pos1), 
            Selection::new(Position::new(0, 0))
        ]);
    }


}