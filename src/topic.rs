pub mod topics {
    use iced::advanced::{renderer, Widget};

    pub struct Topic {

    }
    impl Topic {

    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Topic
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> iced::Size<iced::Length> {
            todo!()
        }
    
        fn layout(
            &self,
            tree: &mut iced::advanced::widget::Tree,
            renderer: &Renderer,
            limits: &iced::advanced::layout::Limits,
        ) -> iced::advanced::layout::Node {
            todo!()
        }
    
        fn draw(
            &self,
            tree: &iced::advanced::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: iced::advanced::Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            viewport: &iced::Rectangle,
        ) {
            todo!()
        }
    }
}