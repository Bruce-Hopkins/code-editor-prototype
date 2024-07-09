// use iced::{advanced::graphics::futures::backend::default::Executor, alignment, widget::{button, column, container, row, scrollable, text, text_input, Column}, Alignment, Application, Element, Length, Padding, Sandbox, Settings, Theme};
// // use rfd;

// struct AlchemyEditor {

// }

// impl Application for AlchemyEditor {
//     type Executor = Executor;

//     type Message = ();

//     type Theme = Theme;

//     type Flags = ();

//     fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
//       (Self {}, iced::Command::none())
//     }

//     fn title(&self) -> String {
//         String::from("Alchemy Editor")
//     }

//     fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
//         iced::Command::none()
//     }

//     fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
//         container(
//             column!(
//                 container("Text")
//                 .width(Length::Fill)
//                 .height(Length::Fixed(50.0))
//                 .align_x(alignment::Horizontal::Center)
//                 .align_y(alignment::Vertical::Center),

//                 row!( 
//                     container("Text2")
//                     .width(Length::Fixed(100.0))
//                     .height(Length::Fill)
//                     .align_x(alignment::Horizontal::Center)
//                     .align_y(alignment::Vertical::Center),

//                     container("Text2")
//                     .width(Length::Fill)
//                     .height(Length::Fill)
//                     .align_x(alignment::Horizontal::Center)
//                     .align_y(alignment::Vertical::Center),

//                 ),

//                 container("Text3")
//                 .width(Length::Fill)
//                 .height(Length::Fixed(50.0))
//                 .align_x(alignment::Horizontal::Center)
//                 .align_y(alignment::Vertical::Center),
//             )
//         )
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .into()
//     }
    
//     fn theme(&self) -> Self::Theme {
//         Self::Theme::default()
//     }
    
//     fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
//         <Self::Theme as iced::application::StyleSheet>::Style::default()
//     }
    
//     fn subscription(&self) -> iced::Subscription<Self::Message> {
//         iced::Subscription::none()
//     }
    
//     fn scale_factor(&self) -> f64 {
//         1.0
//     }
    
// }

// pub fn main() -> iced::Result {
//     let s = String::new();
//     run_app(&s)
// }

// fn run_app<'a>(s: &'a String) -> iced::Result {
//     let setting = Settings::default();
//     AlchemyEditor::run(setting)
// }

pub fn main() {
    
}