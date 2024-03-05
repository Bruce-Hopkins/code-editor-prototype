use iced::{
    advanced::{
        mouse, text,
        widget::{tree, Tree},
        Layout, Widget,
    },
    event, touch, Alignment, Element, Event, Font, Padding, Pixels, Point, overlay,
};

use crate::{core::position::CursorMessage, core::{position::Position, document::Document}};


use super::{textbox::Textbox, floating_text::floating_overlay::FloatingOverlay, view_port::ViewPortMessage};

pub struct TextboxContainer<'a, Message, Renderer>
where
    Renderer: text::Renderer,
{
    child: Element<'a, Message, Renderer>,
    floating_text: Option<Element<'a, Message, Renderer>>,
    textbox: &'a Textbox,
    line_height: f32,
    font_size: f32,
    font: Font,
    longest_line: usize,
}

impl<'a, Message, Renderer> TextboxContainer<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    pub fn new(
        child: Element<'a, Message, Renderer>,
        canvas: &'a Textbox,
        longest_line: usize,
        floating_element: Option<Element<'a, Message, Renderer>>,
        font_size: f32,
        font: Font
    ) -> Self {
        Self {
            textbox: canvas,
            floating_text: floating_element,
            line_height: text::LineHeight::default()
                .to_absolute(Pixels(font_size))
                .0,
            font_size,
            child,
            font,
            longest_line,
        }
    }
}
#[derive(Default)]
struct State {
    dragging: bool
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TextboxContainer<'a, Message, Renderer>
where
    Message: 'a + Clone + CursorMessage + ViewPortMessage,
    Renderer: text::Renderer,
    Renderer: iced::advanced::Renderer,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn children(&self) -> Vec<Tree> {
        if let Some(value) = self.floating_text.as_ref() {
            return vec![Tree::new(&self.child), Tree::new(value)]
        }
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(value) = self.floating_text.as_ref() {
            return tree.diff_children(&[&self.child, &value]);
        }
        tree.diff_children(std::slice::from_ref(&self.child))
    }


    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Renderer>> {
        if let Some(element) = self.floating_text.as_mut() {
            let point = self.textbox.get_window_point_from_position(self.textbox.float_position());
            let overlay = overlay::Element::new(
                layout.position(),
                Box::new(FloatingOverlay::new(element, &mut tree.children[1], layout.bounds().size(), point))
            ); 
            // let overlay = overlay::Element::new(
            //     layout.position(),
            //     Box::new(ModalOverlay::new(element, tree, layout.bounds().size(), None ))
            // ); 
            return Some(overlay)
        }
        self.child.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn width(&self) -> iced::Length {
        iced::Length::Fill
        // self.child.as_widget().width()
    }

    fn height(&self) -> iced::Length {
        // let lines:Vec<&str> = self.content.lines().collect();
        // let height_line = lines.len() as f32 * self.line_height;
        // iced::Length::Fixed(10.0)
        self.child.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let width = match self.longest_line {
            0 => 1280.0,
            value => {
                let text_width = value as f32 * self.textbox.text_width();
                if text_width > 1280.0 {
                    text_width
                } else {
                    1280.0
                }
            },
        };


        let limits = limits.width(width).loose();

        let mut content = self.child.as_widget().layout(renderer, &limits);
        let padding = Padding::ZERO;
        let size = content.size();
        // let size = limits.pad(padding).resolve(content.size());

        content.move_to(Point::new(padding.left, padding.top));
        content.align(
            Alignment::Start,
            Alignment::Start,
            size,
        );

        iced::advanced::layout::Node::with_children(size.pad(padding), vec![content])
        // self.child.as_widget().layout(renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.child.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::event::Status {
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                state.dragging = true;
                if cursor.is_over(bounds) {

                    let text_width = self.textbox.text_width();
                    
                    let x = cursor.position().unwrap().x - bounds.x;
                    let point = Point::new(x, cursor.position().unwrap().y - bounds.y);
                    let cursor = line_hit_test(self.textbox.buffer().buffer(), self.line_height, text_width, point);
                    shell.publish(Message::from_cursor_position(cursor));
                    shell.publish(Message::set_textbox_focus(true));
                } else {
                    shell.publish(Message::set_textbox_focus(false));
                }

            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                state.dragging = false;

                // So that components like the scrollbar continue to work we send the event `Ignored`
                return event::Status::Ignored;
            }
            Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                if state.dragging { 
                    if let Some(point) = cursor.position() {
                        let x = point.x - bounds.x;
                        let point = Point::new(x, point.y - bounds.y);
        
                        let text_width = renderer.measure_width(
                            "T",
                            self.font_size,
                            self.font.into(),
                            text::Shaping::Basic,
                        );
                        let cursor = line_hit_test(self.textbox.buffer().buffer(), self.line_height, text_width, point);
                        shell.publish(Message::from_selection_move(cursor))
                    }
                }
            }
            _ => return event::Status::Ignored,
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        let mouse_is_over = cursor.is_over(layout.bounds());
        if mouse_is_over {
            return iced::mouse::Interaction::Text;
        }
        iced::mouse::Interaction::default()
    }

    fn draw(
        &self,
        state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.child.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        )
    }
}

impl<'a, Message, Renderer> From<TextboxContainer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced::advanced::renderer::Renderer + iced::advanced::text::Renderer + 'a,
    Message: 'a + Clone + CursorMessage + ViewPortMessage,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    fn from(text_input: TextboxContainer<'a, Message, Renderer>) -> Self {
        Self::new(text_input)
    }
}

fn monospace_hit_test(text: &str, glyph_width: f32, point: Point) -> f32 {
    let result = (point.x / glyph_width).floor();
    if result > text.len() as f32 {
        return text.len() as f32;
    }
    result
}

fn line_hit_test(text: &Document, line_height: f32, glyph_width: f32, point: Point) -> Position {
    let mut line_num = (point.y / line_height).floor() as usize;


    let line = text
        .get_line(line_num)
        .unwrap_or_else(|| {
            line_num = text.len();
            text.last_line().unwrap()
        });
    let character = monospace_hit_test(line.as_str().unwrap_or(&line.to_string()), glyph_width, point) as usize;

    Position::new(line_num, character)
}
