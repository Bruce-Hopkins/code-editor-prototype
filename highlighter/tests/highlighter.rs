#[cfg(test)]
mod highlighter_tests {
    use highlighter::highlighter::{HighlightItem, Highlighter, HighlighterConfig};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn should_highlight_code() {
        let document_string = "fn main() {\nlet i = 0;\n}".to_owned();
        let config = HighlighterConfig::rust_config(&document_string);
        let mut highlighter = Highlighter::new(&config, 0..usize::MAX, &document_string);
        
        let highlighter_iter = highlighter.highlighter_iter();
        let highlight_items:Vec<HighlightItem> = highlighter_iter.collect();
        
        assert_eq!(highlight_items.len(), 10);
        assert_eq!(highlight_items[0].range.end_byte, 2);
        assert_eq!(highlight_items[9].range.end_byte, 24);
    }
}

