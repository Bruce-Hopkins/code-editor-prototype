use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self};
use iced::advanced::{self, Clipboard, Shell};

use iced::event;
use iced::mouse;
use iced::{
    Element, Event, Length, Point, Rectangle, Size,
};
use crate::widgets::view_port::ViewPortMessage;



pub struct FloatingOverlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut widget::Tree,
    size: Size,
    point: Point
}

impl<'a, 'b, Message, Renderer> FloatingOverlay<'a, 'b, Message, Renderer> {
    pub fn new(content: &'b mut Element<'a, Message, Renderer>, tree: &'b mut widget::Tree, size: Size, point: Point) -> Self {
        Self {
            content,
            tree,
            size,
            point
        }
    }
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for FloatingOverlay<'a, 'b, Message, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone + ViewPortMessage,
{
    fn layout(
        &self,
        renderer: &Renderer,
        _bounds: Size,
        position: Point,
    ) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let child = self
            .content
            .as_widget()
            .layout( renderer, &limits);

        let point = Point {x: position.x + self.point.x, y: position.y + self.point.y};

        let mut node = layout::Node::with_children(self.size, vec![child]);
        node.move_to(point);

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let content_bounds = layout.children().next().unwrap().bounds();

        if let Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        )) = &event
        {
            if !cursor.is_over(content_bounds) {
                shell.publish(Message::dismiss_modal());
                return event::Status::Captured;
            }
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
        )
    }
}
