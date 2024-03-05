use iced::widget::scrollable::{Id, Properties};
use iced::widget::{column, container, scrollable , text};
use iced::{Element};
use iced::{alignment, theme, Background, BorderRadius, Color, Length, Padding, Pixels, Theme};
use crate::Message;

pub mod modal;
pub mod textbox_container;
pub mod textbox;
pub mod view_port;
pub mod main_view;
pub mod floating_text;
pub mod layout;

struct CodeLineTheme;

impl container::StyleSheet for CodeLineTheme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            background: Some(Background::from(Color::TRANSPARENT)),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            text_color: Some(Color::from_rgb8(153, 153, 153)),
        }
    }
}



pub fn line_number(number_of_lines: usize, font_size: f32, height: f32, id: Id) -> Element<'static, Message> {
    let mut lines: Vec<Element<'static, Message>> = Vec::new();
    let box_height = text::LineHeight::default().to_absolute(Pixels(font_size)).0;
    for i in 1..number_of_lines.saturating_add(1) {
        let padding = Padding {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 5.0,
        };

        let container = container(text(i).size(font_size))
            .center_x()
            .align_y(alignment::Vertical::Top)
            .width(Length::Fixed(80.0))
            .padding(padding)
            .height(box_height);
        lines.push(container.into())
    }
    let theme = Box::new(CodeLineTheme);
    scrollable(
        container(column(lines)).style(theme::Container::Custom(theme))
        .height(height)
    )
        .id(id)
        .direction(scrollable::Direction::Vertical(
            Properties::default().scroller_width(0.0).width(0.0),
        ))
        .height(Length::Fill)
        .into()
}
