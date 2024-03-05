use iced::{widget::{text, container, row, button, column}, Element, Length, Padding, Pixels};
use iced_style::theme::{self};

use crate::{styles::{button::MenuButton, container::MenuContainer, text::TextSaved}, widgets::main_view::MainView, Message};





pub fn layout<'a> (child: Element<'a, Message>, modal: Option<Element<'a, Message>>, is_saved: bool)->  Element<'a, Message> 
{
    let theme = Box::new(MenuContainer);
    
    column!(
        container(navbar(is_saved)).width(Length::Fill)
        .style(theme::Container::Custom(theme.clone())),
        main_view(child, modal),
    ).into()
}

fn navbar(is_saved: bool) -> Element<'static, Message>{
    let theme = Box::new(MenuButton);
    let text_container = Box::new(TextSaved);
    let mut row = row!(
        button(text("Open File").size(Pixels::from(14.0))).style(theme::Button::Custom(theme.clone()))
        .on_press(Message::SelectFile)
        .padding(Padding::from([7, 12, 10, 12])),

        button(text("Open Folder").size(Pixels::from(14.0))).style(theme::Button::Custom(theme.clone()))
        .padding(Padding::from([7, 12, 10, 12]))
        .on_press(Message::SelectFolder),

        button(text("New File").size(Pixels::from(14.0))).style(theme::Button::Custom(theme.clone()))
        .padding(Padding::from([7, 12, 10, 12]))
        .on_press(Message::NewFile),

        button(text("Save").size(Pixels::from(14.0)))
        .style(theme::Button::Custom(theme.clone()))
        .padding(Padding::from([7, 12, 10, 12]))
        .on_press(Message::Save),

    )
    .padding(Padding::from([0, 0, 0, 15]))
    .align_items(iced::Alignment::Start);

    if !is_saved {
        row = row.push(
            container(
                text("Unsaved Changes")
                .size(10.0)
            ).style(theme::Container::Custom(text_container))
            .padding(Padding::from([12, 12, 7, 40]))
        )
    }
    row.into()
}


fn main_view<'a> (child: Element<'a, Message>, modal: Option<Element<'a, Message>>) -> Element<'a, Message> {
    MainView::new(child, modal).into()
}
