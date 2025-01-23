use iced::{advanced::{graphics::{core::Element, futures::backend::default}, Widget}, alignment, widget::{self, button, center, container, mouse_area, opaque, row, stack, text, text_input, Button, Column, Container, Row, Space, Text}, Alignment, Background, Color, Length, Renderer, Settings, Shadow, Size, Task, Theme};
use iced_shapes::{circle::Circle, rectangle::Rectangle};
use iced_aw::{card, color_picker, menu::{self, Item, Menu}, menu_bar, style};
use iced_aw::menu_items;
use iced::widget::column;
use flashcard_rs::Pin;

pub fn main() -> iced::Result {
    iced::run("Title", App::update, App::view)
}

#[derive(Default)]
struct App {
    show_modal: Popups,
    card: Flashcard,
}

#[derive(Default)]
struct Flashcard {
    bg_color: Option<Background>,
    header: String,
    footer: String,
    content: String,
}
// impl Default for Card {
//     fn default() -> Self {
//         Self {
//             color: Color::from_rgb(0.5, 0.5, 0.5),
//         }
//     }
// }

#[derive(Debug, Default, Clone)]
enum Popups {
    Flashcards,
    Assist,
    Test,
    ColorPicker,
    Text,
    #[default]
    None
}

#[derive(Debug, Clone)]
enum Message {
    Debug(String),
    Flashcards,
    Test,
    Assist,
    NoPopup,
    ColorPicker,
    CancelColor,
    ChooseColor,
    SubmitColor(Color),
    ContentChanged(String),
    Text,
    // ShowModal,
    // HideModal,
    None
}

impl App {
    fn hide_modal(&mut self) {
        self.show_modal = Popups::None;
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Flashcards => {
              self.show_modal = Popups::Flashcards;
            },
            // Message::ShowModal => {
            //     self.show_modal = true;
            //     widget::focus_next::<text_input::TextInput<Message>>();
            // }
            // Message::HideModal => {
            //     self.hide_modal();
            //     Task::<Message>::none();
            // },
            Message::Debug(_) => {},
            Message::Test => {
                self.show_modal = Popups::Test;
            },
            Message::Assist => {},
            Message::None => {},
            Message::NoPopup => {
                self.show_modal = Popups::None
            },
            Message::ColorPicker => {
                self.show_modal = Popups::ColorPicker
            },
            Message::CancelColor => {

            },
            Message::ChooseColor => {

            },
            Message::SubmitColor(color) => {
                self.card = Flashcard {
                    bg_color: Some(Background::Color(color)),
                    content: self.card.content.clone(),
                    ..Default::default()
                }
            },
            Message::Text => {
                self.show_modal = Popups::Text;
            },
            Message::ContentChanged(content) => {
                println!("{}", content);
                self.card.content = content;
            },
        }
    }
    fn view(&self) -> Container<Message> { 
        let rect: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 1000.0).style(Color::WHITE).into();
        let rect2: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 1000.0).style(Color::WHITE).into();
        let button_style = iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgb8(43, 43, 43))), 
            ..Default::default()
        };


    
        let menu1 = MenuButton::new("Nested Menus", button_style);
        let menu_bar = menu_bar!(
            (menu1.debug_button_s(), {
                let sub_menu =  Menu::new(menu_items!(
                    (menu1.debug_button("E".to_string()))
                    (menu1.debug_button("E".to_string()))
                    (menu1.debug_button("E".to_string()))
                    (menu1.debug_button("E".to_string()))
                    (menu1.debug_button("E".to_string()))
                )).max_width(180.0).offset(15.0).spacing(5.0);
                sub_menu
            })
        ); 
        
        let header1: Element<'_, Message, Theme, Renderer> = Rectangle::new(1000.0, 30.0).style(Color::from_rgb8(81, 80, 80)).into();
        // let header2: Element<'_, Message, Theme, Renderer> = Rectangle::new(1000.0, 50.0).style(Color::WHITE).into();

        let main_container: Element<'_, Message, Theme, Renderer> = container(
            column!(
                container(
                    row!(
                        Space::new(20.0, 0.0),
                        column!(
                            Space::new(0.0, 3.0),
                            menu_bar
                        )
                    )
                )
                .width(Length::Fixed(1000.0))
                .height(Length::Fixed(40.0))
                .style(move |_: &iced::Theme| iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgb8(81, 80, 80))),
                    ..Default::default()
                 }),
                 Space::new(0.0, 30.0),
                 row!(
                    Space::new(70.0, 0.0),
                    container(
                        column!(
                            Text::new("Learn & explore"),
                            Space::new(0.0, 15.0),
                            Button::new("Flashcards").on_press(Message::Flashcards),
                            Space::new(0.0, 10.0),
                            Button::new("Test").on_press(Message::Test),
                            Space::new(0.0, 10.0),
                            Button::new("Assist").on_press(Message::Assist)
                            
                        )
                    )
                    .width(Length::Fixed(110.0))
                    .height(Length::Fixed(300.0))
                    .style(move |_: &iced::Theme| iced::widget::container::Style {
                        background: Some(Background::Color(Color::from_rgb8(43, 43, 43))),
                        ..Default::default()
                    })
                    ,
                    Space::new(60.0, 0.0),
                    container(
                        column!(
                        header1,
                        Space::new(0.0, 20.0),
                        row!(
                            Space::new(7.5, 0.0),
                            container(
                                column!(
                                        // Space::new(Length::Fixed(0.0), Length::Fixed(50.0)),
                                        Button::new("Text")
                                            .style(|_theme: &Theme, _status| {
                                                iced::widget::button::Style {
                                                    background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                    text_color: Color::from_rgb8(255, 255, 255),
                                                    ..Default::default()
                                                }
                                            })
                                            .on_press(Message::Text),
                                            
                                
                                        Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                                        Button::new("Color")
                                            .style(|_theme: &Theme, _status| {
                                                iced::widget::button::Style {
                                                    background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                    text_color: Color::from_rgb8(255, 255, 255),
                                                    ..Default::default()
                                                }
                                            })
                                            .on_press(Message::ColorPicker)
                                            ,
                                            
                                
                                        Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                                        Button::new("Image")
                                            .style(|_theme: &Theme, _status| {
                                                iced::widget::button::Style {
                                                    background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                    text_color: Color::from_rgb8(255, 255, 255),
                                                    ..Default::default()
                                                }
                                            }),
                            )),
                            Space::new(7.5, 0.0),
                            //.width(
                            //    Length::Fixed(60.0)
                            //)
                            column!(
                                //header2,
                                // Space::new(0.0, 50.0),
                                card(
                                    "Test",
                                    Column::new().push(
                                        "Test"
                                    )
                                    .push(Space::new(0.0, 50))
                                )
                                .style(|_theme: &Theme, _status|  style::card::Style {
                                    head_background: self.card.bg_color.unwrap_or(Background::Color(Color::WHITE)),
                                    ..Default::default()
                                })
                                .width(Length::Fixed(400.0))
                            )
                        )
                    )
                    )
                    .width(Length::Fixed(500.0))
                    .height(Length::Fixed(300.0))
                    .style(move |_: &iced::Theme| iced::widget::container::Style {
                        background: Some(Background::Color(Color::from_rgb8(43, 43, 43))),
                        ..Default::default()
                    })
                    .align_x(Alignment::Center)
                )
            )
        )
            .width(Length::Fixed(800.0))
            .height(Length::Fixed(1000.0))
            .style(move |_: &iced::Theme| iced::widget::container::Style {
                background:Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                ..Default::default()
             })
        .into();
    
        let main_container =  container(
            row![
                rect,
                main_container,
                rect2
            ]
        );
        let background_rect: Element<'_, Message, Theme, Renderer> = Rectangle::new(700.0, 340.0)
        .style(Color::from_rgb8(81, 80, 80))
        .border_radius(20.0)
        .into();

        match self.show_modal {
            
            Popups::Flashcards => {
                let Rect: Element<'_, Message, Theme, Renderer> = Rectangle::new(200.0, 200.0)
                .style(Color::WHITE)
                .into(); 
                container(
                    modal(
                        main_container,
                        container(stack![
                            Rect,
                            container(Button::new("Test").on_press(Message::NoPopup)).center_x(Length::Fill)
                        ]),
                        Message::None
                    )
                )
            },
            Popups::None => {
                main_container
            },
            Popups::Assist => todo!(),
            Popups::Test => {
                let Rect: Element<'_, Message, Theme, Renderer> = Rectangle::new(200.0, 200.0)
                .style(Color::WHITE)
                .into(); 
                container(
                    modal(
                        main_container,
                        container(stack![
                            Rect,
                            container(Button::new("Test").on_press(Message::NoPopup)).center_x(Length::Fill)
                        ]),
                        Message::None
                    )
                )
            },
            Popups::ColorPicker => {
                container(
                    modal(
                        main_container,
                        container(
                            stack![
                            background_rect,
                            row!(
                                Pin::new(
                                    // row!(
                                        color_picker(
                                            true,
                                            Color::BLACK,
                                            Button::new(Text::new("Set Color")).on_press(Message::ChooseColor),
                                            Message::CancelColor,
                                            Message::SubmitColor,
                                        ),
                                    
                                // ) Button::new("Exit").on_press(Message::NoPopup)
                                )
                                .x(280)
                                .y(150),
                                Button::new("Exit").on_press(Message::NoPopup)
                            )
                     
                            ]
                        ),
                        Message::None
                    )
                )
            },
            Popups::Text => {
                container(
                    modal(
                        main_container,
                        container(
                            stack![
                                background_rect,
                                row!(
                                    Pin::new(
                                        container(
                                            text_input("Type something here..", &self.card.content)
                                                .on_input(Message::ContentChanged)
                                        )
                                    )           
                                    .x(280)
                                    .y(150),
                                )
                            ]
                        ),
                        Message::None
                    )
                )
            },
        }
    }
    
        
}

#[derive(Copy, Clone)] 
struct MenuButton {
    main_label: &'static str, 
    style: Option<iced::widget::button::Style>,
}

impl MenuButton {
    fn new(label: &'static str, style: iced::widget::button::Style) -> Self {
        Self {
            main_label: label,
            style: Some(style),
        }
    }

    fn labeled_button(self, label: String, msg: Message) -> button::Button<'static, Message, iced::Theme, iced::Renderer> {
        Self::base_button(self, text(label).align_y(alignment::Vertical::Center), msg)
    }

    fn debug_button(self, label: String) -> button::Button<'static, Message, iced::Theme, iced::Renderer> {
        Self::labeled_button(self, label.clone(), Message::Debug(label.clone())).width(Length::Fill)
    }

    fn debug_button_s(self) -> button::Button<'static, Message, iced::Theme, iced::Renderer> {
        let label = self.main_label; // No clone needed since &'static str is Copy
        Self::labeled_button(self, label.to_string(), Message::Debug(label.to_string())).width(Length::Shrink)
    }

    fn base_button<'a>(self, content: impl Into<Element<'a, Message, Theme, Renderer>>, msg: Message) -> button::Button<'a, Message> {
        button(content)
            .padding([4, 8])
            .style(move |theme: &Theme, status: iced::widget::button::Status| self.style.unwrap())
            .on_press(msg)
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    on_blur: Message,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}