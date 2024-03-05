use iced::{Background, Color, BorderRadius};
use iced_style::{container, Theme};

#[derive(Debug, Clone)]

pub struct MenuContainer;

impl container::StyleSheet for MenuContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::from(Color::from_rgba8(25, 31, 43, 1.0))),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Some(Color::WHITE),
        }
    }
}

#[derive(Debug, Clone)]

pub struct NormalContainer;

impl container::StyleSheet for NormalContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            background: Some(Background::from(Color::from_rgb(
                0x20 as f32 / 255.0,
                0x22 as f32 / 255.0,
                0x25 as f32 / 255.0,
            ))),
            border_radius: BorderRadius::from(0.0),
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            text_color: Some(Color::WHITE),
        }
    }
}

#[derive(Debug, Clone)]

pub struct FloatingContainer;

impl container::StyleSheet for FloatingContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            background: Some(Background::from(Color::from_rgb(
                0.0,
                0.0,
                0.0,
            ))),
            border_radius: BorderRadius::from(0.0),
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            text_color: Some(Color::WHITE),
        }
    }
}