//! A pin widget positions a widget at some fixed coordinates inside its boundaries.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::core::Length::Fill; }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::pin;
//! use iced::Fill;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     pin("This text is displayed at coordinates (50, 50)!")
//!         .x(50)
//!         .y(50)
//!         .into()
//! }
//! ```
//!
use iced::advanced::layout;
use iced::advanced::widget::tree;
use iced::advanced::widget::Operation;
use iced::advanced::widget::Tree;
use iced::mouse;
use iced::overlay;

use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::{self, Element, Event, Length, Pixels, Point, Rectangle, Size, Vector};
/// A widget that positions its contents at some fixed coordinates inside of its boundaries.
///
/// By default, a [`Pin`] widget will try to fill its parent.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::core::Length::Fill; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::pin;
/// use iced::Fill;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     pin("This text is displayed at coordinates (50, 50)!")
///         .x(50)
///         .y(50)
///         .into()
/// }
/// ```
#[allow(missing_debug_implementations)]
pub struct Pin<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    width: Length,
    height: Length,
    position: Point,
}

impl<'a, Message, Theme, Renderer> Pin<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a [`Pin`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            width: Length::Fill,
            height: Length::Fill,
            position: Point::ORIGIN,
        }
    }

    /// Sets the width of the [`Pin`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Pin`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the position of the [`Pin`]; where the pinned widget will be displayed.
    pub fn position(mut self, position: impl Into<Point>) -> Self {
        self.position = position.into();
        self
    }

    /// Sets the X coordinate of the [`Pin`].
    pub fn x(mut self, x: impl Into<Pixels>) -> Self {
        self.position.x = x.into().0;
        self
    }

    /// Sets the Y coordinate of the [`Pin`].
    pub fn y(mut self, y: impl Into<Pixels>) -> Self {
        self.position.y = y.into().0;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Pin<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let available = limits.max() - Size::new(self.position.x, self.position.y);

        let node = self
            .content
            .as_widget()
            .layout(tree, renderer, &layout::Limits::new(Size::ZERO, available))
            .move_to(self.position);

        let size = limits.resolve(self.width, self.height, node.size());
        layout::Node::with_children(size, vec![node])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget().operate(
            tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    // fn update(
    //     &mut self,
    //     tree: &mut widget::Tree,
    //     event: Event,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     renderer: &Renderer,
    //     clipboard: &mut dyn Clipboard,
    //     shell: &mut Shell<'_, Message>,
    //     viewport: &Rectangle,
    // ) {
    //     self.content.as_widget_mut().update(
    //         tree,
    //         event,
    //         layout.children().next().unwrap(),
    //         cursor,
    //         renderer,
    //         clipboard,
    //         shell,
    //         viewport,
    //     );
    // }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            self.content.as_widget().draw(
                tree,
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                &clipped_viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }

    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        _event: Event,
        _layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        iced::advanced::graphics::core::event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<Pin<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(pin: Pin<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(pin)
    }
}
