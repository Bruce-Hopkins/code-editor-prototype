use std::ops;
use ropey::{RopeSlice};
use tree_sitter::{InputEdit, Parser, Query, QueryCursor, Range, Tree};

use crate::core::{document::Document, position::Position};
pub mod color_selector;

pub struct HighlighterConfig {
    tree: Tree,
    query: Query,
    parser: Parser,
}

impl HighlighterConfig {
    pub fn new(tree: Tree, query: Query, parser: Parser) -> Self {
        Self {
            tree,
            query,
            parser,
        }
    }

    pub fn rust_config(source_code: &str) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading Rust grammar");
        let tree = parser.parse(source_code, None).unwrap();

        let query = Query::new(
            tree_sitter_rust::language(),
            include_str!("../../queries/highlights.scm"),
        )
        .unwrap();

        Self::new(tree, query, parser)
    }

    fn edit(&mut self, input: &InputEdit, content: &RopeSlice) {
        self.tree.edit(input);
        let tree = self.parser.parse(content.to_string(), Some(&self.tree));
        self.tree = tree.unwrap()
    }

    pub fn delete(&mut self, start_byte: usize, end_byte: usize, start_pos: Position, end_pos: Position, content:&RopeSlice) {
        let input = tree_sitter::InputEdit {
            start_byte: start_byte + 1,
            old_end_byte: end_byte,
            new_end_byte: start_byte,
            start_position: tree_sitter::Point {
                row: start_pos.line(),
                column: start_pos.character() + 1,
            },
            old_end_position: tree_sitter::Point {
                row: end_pos.line(),
                column: end_pos.character(),
            },
            new_end_position: tree_sitter::Point {
                row: start_pos.line(),
                column: start_pos.character(),
            },
        };
        self.edit(&input, content)
    }

    pub fn insert(&mut self, start_byte: usize, end_byte: usize, start_pos: Position, end_pos: Position, length: usize, content:&RopeSlice) {
        let input = tree_sitter::InputEdit {
            start_byte,
            old_end_byte: end_byte,
            new_end_byte: start_byte + length,
            start_position: tree_sitter::Point {
                row: start_pos.line(),
                column: start_pos.character(),
            },
            old_end_position: tree_sitter::Point {
                row: end_pos.line(),
                column: end_pos.character(),
            },
            new_end_position: tree_sitter::Point {
                row: start_pos.line(),
                column: start_pos.character() + 1,
            },
        };
        self.edit(&input, content)
    }

}

#[derive(Debug, Clone)]
pub struct HighlightItem {
    pub capture_name: String,
    pub kind: String,
    pub range: Range,
}
#[derive(Debug, Clone)]
pub struct Highlighter {
    pub captures: Vec<HighlightItem>,
}

impl Highlighter {
    pub fn new(config: &HighlighterConfig, range: ops::Range<usize>, buffer: &Document) -> Option<Self> {
        let capture_names = config.query.capture_names();

        let root_node = config.tree.root_node();

        // println!("Range is {:?}", range);
        let mut cursor = QueryCursor::new();
        cursor.set_byte_range(range);

        let content = buffer.to_string();
        let captures = cursor.captures(&config.query, root_node, content.as_bytes());

        let mut highlight_items: Vec<HighlightItem> = Vec::new();
        for (q_match, _i) in captures {
            for cap in q_match.captures {
                let item: HighlightItem = HighlightItem {
                    capture_name: capture_names.get(cap.index as usize)?.to_string(),
                    kind: cap.node.kind().to_string(),
                    range: cap.node.range(),
                };
                highlight_items.push(item)
            }
        }

        Some(Self {
            captures: highlight_items,
        })
    }
}
