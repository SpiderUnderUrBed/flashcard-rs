pub mod rectangle {
    use iced::advanced::renderer;
    use iced::{
        advanced::{
            layout::{self, Layout},
            widget::{self, Widget},
        },
        border, Color, Element, Renderer, Shadow, Theme,
    };
    use iced::{Length, Point, Rectangle as IcedRectangle, Size};

    pub struct Rectangle {
        width: f32,
        length: f32,
        background_color: Color,
        border_radius: f32,
    }

    impl Rectangle {
        pub fn new(width: f32, length: f32) -> Self {
            Self {
                width,
                length,
                background_color: Color::BLACK,
                border_radius: 0.0,
            }
        }
        pub fn style(mut self, color: Color) -> Self {
            self.background_color = color;
            self
        }
        pub fn border_radius(mut self, border_radius: f32) -> Self {
            self.border_radius = border_radius;
            self
        }
    }

    impl<Message: 'static> Into<Element<'_, Message, Theme, Renderer>> for Rectangle
    where
        Renderer: renderer::Renderer,
    {
        fn into(self) -> Element<'static, Message> {
            Element::new(self)
        }
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rectangle
    where
        Renderer: renderer::Renderer,
    {
        fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            let size = limits.resolve(self.width, self.length, Size::new(self.width, self.length));

            layout::Node::new(size)
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor_position: iced::mouse::Cursor,
            _viewport: &IcedRectangle,
        ) {
            let bounds = layout.bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: border::rounded(self.border_radius),
                    shadow: Shadow::default(),
                },
                iced::Background::Color(self.background_color),
            );
        }

        fn size(&self) -> Size<Length> {
            Size {
                width: iced::Length::Fixed(self.width),
                height: iced::Length::Fixed(self.length),
            }
        }
    }
}
