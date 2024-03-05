use iced::{Background, Color, BorderRadius};
use iced_style::{container, Theme};

pub struct TextSaved;

impl container::StyleSheet for TextSaved {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::from(Color::from_rgba8(25, 31, 43, 1.0))),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Some(Color::from_rgb8(130, 130, 130)),
        }
    }
}