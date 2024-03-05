use iced::{Element, widget::{text, container, column, scrollable::{self, Properties}}};
use iced_style::theme;

use crate::{core::position::Position, Message, styles::container::FloatingContainer};


#[derive(Clone, Debug)]
pub struct FloatingElement {
    pub view_box: FloatingText,
    pub position: Position
}
#[derive(Clone, Debug)]
pub enum FloatingText {
    Diagnostic(String)
}

impl FloatingText {
    pub fn show(&self) -> Element<'static, Message> {
        match self {
            FloatingText::Diagnostic(value) => {
                let mut text_lines = Vec::new();
                for line in value.lines() {
                    let text = text(line).into();
                    text_lines.push(text);
                }


                let theme = Box::new(FloatingContainer);
                container(
                    iced::widget::scrollable(
                        column(text_lines).width(iced::Length::Fill)
                    ).direction(scrollable::Direction::Vertical(
                        Properties::default().scroller_width(7.0).width(7.0),
                    ))
                )
                .padding(20)
                .style(theme::Container::Custom(theme))
                .max_height(400)
                .max_width(500)
                .into()
            }
        }
    }
}