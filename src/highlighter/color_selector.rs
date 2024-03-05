use crate::highlighter::HighlightItem;
use iced::Color;
use std::vec::IntoIter;

use crate::core::position::Position;

pub struct ColorSelector;

impl ColorSelector {
    /**
     * Program uses the color theme from one-dark and is under the following license:

        Copyright (c) 2016 GitHub Inc.

        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
     */
    pub fn get_highlighter_color(&self, capture_name: &str) -> Color {
        match capture_name {
            "type" | "type.builtin" => {
                Color::from_rgb8(229, 192, 123)
            },
            "constant" | "constant.builtin" => {
                Color::from_rgb8(209, 154, 102)
            }
            "constructor" | "function" | "function.macro" | "function.method" => {
                Color::from_rgb8(97, 175, 239)
            }
            "comment" => Color::from_rgb8(153, 153, 153),
            "string" => Color::from_rgb8(110, 255, 89),
            "punctuation.bracket" | "punctuation.delimiter" => Color::from_rgb8(190, 190, 190),
            "variable.builtin" | "variable.parameter" => Color::from_rgb8(224, 108, 117),
            "keyword" | "operator" => Color::from_rgb8(198, 120, 221),
            _ => Color::from_rgb8(171, 178, 191),
        }
    }

    pub fn default_text_color(&self) -> Color {
        Color::from_rgb8(171, 178, 191)
    }

    /**
     * Selects a color based on the highlighted item.
     *
     * If at any time the highlighted item is `None` what means the iterator is finished, so we return the default color.
     *
     * If the highlighted item is within range of the position of the character, it returns the color for the capture name
     *
     * if the highlighted item's `end_point` is less than the character position, that means we need to update the item to be highlighted, because we passed it.
     * change the highlight item to be the next item in the iterator`and try again with the same values.
     *
     * if the highlighted item's `start_point` is more than the character position, then this item has no highlighting. So we use the default coloring.
     *
     * If none of these conditions are somehow met, we try to get the next highlighted item.
     */
    pub fn select(
        &self,
        highlighter: &mut IntoIter<HighlightItem>,
        item: &mut Option<HighlightItem>,
        pos: &Position,
    ) -> Color {
        loop {
            if let Some(value) = &item {
                let start_point = Position::from(value.range.start_point);
                let end_point = Position::from(value.range.end_point);
                if pos.is_within(start_point, end_point) {
                    return self.get_highlighter_color(&value.capture_name);
                }
                // If the endpoint is less than the pos then run next()
                if pos > &end_point {
                    *item = highlighter.next();
                    continue;
                }
                if pos < &start_point {
                    return self.default_text_color();
                }
                *item = highlighter.next()
            } else {
                return self.default_text_color();
            }
        }
    }
}
#[cfg(test)]
mod color_selector_tests {
    use crate::{highlighter::HighlightItem, core::position::Position};
    use iced::Color;
    use pretty_assertions::assert_eq;
    use tree_sitter::Range;

    use super::ColorSelector;

    #[test]
    fn test_color_selector_past_range() {
        // Setup highlighter
        let item1 = HighlightItem {
            capture_name: String::from("constant"),
            kind: String::from("Does not matter"),
            range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: tree_sitter::Point { row: 0, column: 0 },
                end_point: tree_sitter::Point { row: 0, column: 0 },
            },
        };
        let highlighter_items = vec![item1];
        let mut highlighter = highlighter_items.into_iter();
        let mut current_item = highlighter.next();

        // Set the editor position
        let pos = Position::new(
            0,
            0
        );

        // Select a color
        let color_selector = ColorSelector;
        let color = color_selector.select(&mut highlighter, &mut current_item, &pos);

        // Should try to get the next item, which will become `None``
        assert!(current_item.is_none());

        // Should use the default color
        assert_eq!(color, Color::from_rgb8(255, 128, 128));
    }

    #[test]
    fn test_color_selector_within_range() {
        // Setup highlighter
        let item1 = HighlightItem {
            capture_name: String::from("constant"),
            kind: String::from("Does not matter"),
            range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: tree_sitter::Point { row: 0, column: 0 },
                end_point: tree_sitter::Point { row: 0, column: 3 },
            },
        };
        let highlighter_items = vec![item1];
        let mut highlighter = highlighter_items.into_iter();
        let mut current_item = highlighter.next();

        // Set the editor position
        let pos = Position::new(
            0,
            0
        );

        // Select a color
        let color_selector = ColorSelector;
        let color = color_selector.select(&mut highlighter, &mut current_item, &pos);

        // Should try to get the next item, which will become `None``
        assert!(current_item.is_some());

        // Should use the default color
        assert_eq!(color, Color::from_rgb8(255, 143, 0));
    }

    #[test]
    fn test_color_selector_before_range() {
        // Setup highlighter
        let item1 = HighlightItem {
            capture_name: String::from("constant"),
            kind: String::from("Does not matter"),
            range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: tree_sitter::Point { row: 0, column: 5 },
                end_point: tree_sitter::Point { row: 0, column: 10 },
            },
        };
        let highlighter_items = vec![item1];
        let mut highlighter = highlighter_items.into_iter();
        let mut current_item = highlighter.next();

        // Set the editor position
        let pos = Position::new(
            0,
            0
        );

        // Select a color
        let color_selector = ColorSelector;
        let color = color_selector.select(&mut highlighter, &mut current_item, &pos);

        // We shouldn't change the current item.
        assert!(current_item.is_some());

        // Should use the default color
        assert_eq!(color, Color::from_rgb8(255, 128, 128));
    }
}
