use iced::{Background, BorderRadius, Color, Vector};
use iced_style::{button, Theme};


#[derive(Debug, Clone, Copy)]

pub struct MenuButton;

impl button::StyleSheet for MenuButton {
    type Style = Theme;


    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance { shadow_offset: Vector::default(), background: Some(Background::from(Color::from_rgba8(25, 31, 43, 1.0))), border_radius: BorderRadius::from(0.0), border_width: 0.0, border_color: Color::TRANSPARENT, text_color: Color::WHITE }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance { shadow_offset: Vector::default(), background: Some(Background::from(Color::from_rgba8(70, 77, 89, 1.0))), border_radius: BorderRadius::from(0.0), border_width: 0.0, border_color: Color::TRANSPARENT, text_color: Color::WHITE }
    }

}