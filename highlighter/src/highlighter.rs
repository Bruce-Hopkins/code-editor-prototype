use std::ops;
use ropey::RopeSlice;
use tree_sitter::{InputEdit, Parser, Point, Query, QueryCapture, QueryCaptures, QueryCursor, QueryMatch, Range, Tree};

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

    pub fn edit(&mut self, input: &InputEdit, content: &RopeSlice) {
        self.tree.edit(input);
        let tree = self.parser.parse(content.to_string(), Some(&self.tree));
        self.tree = tree.unwrap()
    }

    pub fn delete(&mut self, start_byte: usize, end_byte: usize, start_pos: Point, end_pos: Point, content:&RopeSlice) {
        let input = tree_sitter::InputEdit {
            start_byte: start_byte + 1,
            old_end_byte: end_byte,
            new_end_byte: start_byte,
            start_position: tree_sitter::Point {
                row: start_pos.row,
                column: start_pos.column + 1,
            },
            old_end_position: tree_sitter::Point {
                row: end_pos.row,
                column: end_pos.column,
            },
            new_end_position: tree_sitter::Point {
                row: start_pos.row,
                column: start_pos.column,
            },
        };
        self.edit(&input, content)
    }

    pub fn insert(&mut self, start_byte: usize, end_byte: usize, start_pos: Point, end_pos: Point, length: usize, content:&RopeSlice) {
        let input = tree_sitter::InputEdit {
            start_byte,
            old_end_byte: end_byte,
            new_end_byte: start_byte + length,
            start_position: tree_sitter::Point {
                row: start_pos.row,
                column: start_pos.column,
            },
            old_end_position: tree_sitter::Point {
                row: end_pos.row,
                column: end_pos.column
            },
            new_end_position: tree_sitter::Point {
                row: start_pos.row,
                column: start_pos.column + 1,
            },
        };
        self.edit(&input, content)
    }

}

#[derive(Debug, Clone)]
pub struct HighlightItem<'tree> {
    pub capture_name: &'tree str,
    pub kind: &'tree str,
    pub range: Range,
}

pub struct Highlighter<'tree> {
    config: &'tree HighlighterConfig,
    content: String,
    cursor: QueryCursor,
}

impl <'tree> Highlighter<'tree> {
    pub fn new<T>(config: &'tree HighlighterConfig, range: ops::Range<usize>, content: &'tree T) -> Self
    where T: 'tree + ToString {
        let mut cursor = QueryCursor::new();
        cursor.set_byte_range(range);

        let content = content.to_string();
        // let captures = cursor.captures(&config.query, root_node, content.as_bytes());
        Highlighter {
            config,
            content,
            cursor,
        }
    }

    pub fn highlighter_iter(&'tree mut self) -> HighlightIter<'tree>{
        HighlightIter::new(self.config, &mut self.cursor, self.content.as_bytes())
    }
}


pub struct HighlightIter<'tree> {
    // config: &'tree HighlighterConfig,
    query_captures: QueryCaptures<'tree, 'tree, &'tree[u8]>,
    capture_names: &'tree [String],
    query_match: Option<QueryMatch<'tree, 'tree>>,
    capture_num: usize
}

// struct HighlighterInterator<'tree, 'a, T>(QueryCaptures<'a, 'tree>);

impl<'tree> HighlightIter<'tree> {
    pub fn new(config: &'tree HighlighterConfig, cursor: &'tree mut QueryCursor, buffer: &'tree [u8]) -> Self {
        let capture_names = config.query.capture_names();
        let root_node = config.tree.root_node();
        let mut captures: QueryCaptures<'tree, 'tree, &[u8]> = cursor.captures(&config.query, root_node, &buffer);
        let query_match = captures.next().map(|(cap, _i)| cap);
        Self {
            // config,
            query_captures: captures,
            capture_names,
            query_match,
            capture_num: 0
        }
    }
}

impl <'tree> Iterator for HighlightIter<'tree> {
    type Item = HighlightItem<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut capture: Option<&QueryCapture<'tree>> = None;
        loop {
            if let Some(query_match) = self.query_match.as_ref() {
                if let Some(value) = query_match.captures.get(self.capture_num) {
                    self.capture_num = self.capture_num.saturating_add(1);
                    capture = Some(value);
                } else {
                    // We try to get the next query_match
                    if let Some((cap, _i)) = self.query_captures.next() {
                        self.capture_num = 0;
                        self.query_match = Some(cap);
                        continue;
                    }
                } 
                
            } 
            break;
        }
        capture.map(|cap| {
            HighlightItem {
                capture_name: &self.capture_names.get(cap.index as usize).unwrap(),
                kind: &cap.node.kind(),
                range: cap.node.range(),
            }
        })
    }
}
