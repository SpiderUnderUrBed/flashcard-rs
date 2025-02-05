use iced::{
    advanced::{
        graphics::core::Element,
        layout::{self, Layout},
        renderer,
        widget::{self, Widget},
    },
    mouse, Background,
};
use iced::{border, Color, Length, Rectangle, Renderer, Shadow, Size, Theme};

pub struct RoundedRectangle {
    width: f32,
    height: f32,
    background_color: Color,
    border_radius: f32,
}

impl RoundedRectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            background_color: Color::BLACK,
            border_radius: 0.0,
        }
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn border_radius(mut self, border_radius: f32) -> Self {
        self.border_radius = border_radius;
        self
    }
}

impl<Message: 'static> From<RoundedRectangle> for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: RoundedRectangle) -> Self {
        Element::new(value)
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RoundedRectangle
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radius),
                shadow: Shadow::default(),
            },
            Background::Color(self.background_color),
        );
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.width),
            height: Length::Fixed(self.height),
        }
    }
}
