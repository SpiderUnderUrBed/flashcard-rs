//#![feature(default_field_values)]


use iced::{advanced::{graphics::{core::Element, futures::backend::default}, Widget}, alignment, widget::{self, button, center, container, mouse_area, opaque, row, scrollable::{self, Rail, Scroller}, stack, text, text_input, Button, Column, Container, Row, Scrollable, Space, Text}, Alignment, Background, Border, Color, Length, Renderer, Settings, Shadow, Size, Task, Theme};
use iced_shapes::{circle::Circle, rectangle::Rectangle};
use iced_aw::{card, color_picker, menu::{self, Item, Menu}, menu_bar, style};
use iced_aw::menu_items;
use iced::widget::column;

mod pin;
// mod topic;

use pin::Pin;

// #[derive(Debug, Clone, Eq, PartialEq, Copy)]
// enum Direction {
//     Vertical,
//     Horizontal,
//     Multi,
// }

pub fn main() -> iced::Result {
    iced::run("Title", App::update, App::view)
}

#[derive(Default)]
struct App {
    show_modal: Popups,
    current_card: Flashcard,
    //topics: Vec<Topic>,
    test: Vec<Topic>,
    current_topic: String,
    expand_questions: bool,
    expand_awnsers: bool,
    //topics: Vec<String> = vec!["E".to_string()],
    // test: &'static str = "Te"
}
#[derive(Debug, Clone)]
struct Topic {
    content: String,
    color: Option<Color>,
    id: u32,
}

#[derive(Default)]
struct Flashcard {
    bg_color: Option<Background>,
    header: String,
    footer: String,
    question: String,
    awnser: String,
    topics: Vec<Topic>
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
    Topics,
    Configure,
    #[default]
    None
}

#[derive(Debug, Clone)]
enum Message {
    Debug(String),
    Flashcards,
    Test,
    Configure,
    Assist,
    NoPopup,
    ColorPicker,
    CancelColor,
    ChooseColor,
    SubmitColor(Color),
    SubmitTopic(Topic),
    UpdateTopic(String),
    SelectTopic(Topic),
    SelectTest(Topic),
    AwnserChanged(String),
    QuestionChanged(String),
    ExpandQuestions,
    ExpandAwnsers,
    Text,
    Topics,
    // ShowModal,
    // HideModal,
    Error,
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
            Message::QuestionChanged(content) => {
                self.current_card.question = content;
            },
            Message::Debug(_) => {},
            Message::Test => {
                self.show_modal = Popups::Test;
            },
            Message::Configure => {
                self.show_modal = Popups::Configure;
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
                self.current_card = Flashcard {
                    bg_color: Some(Background::Color(color)),
                    question: self.current_card.question.clone(),
                    awnser: self.current_card.awnser.clone(),
                    topics: self.current_card.topics.clone(),
                    ..Default::default()
                }
            },
            Message::Text => {
                self.show_modal = Popups::Text;
            },
            Message::Topics => {
                self.show_modal = Popups::Topics;
            },
            Message::SubmitTopic(topic, ) => {
                //if !self.topics.last().unwrap().is_empty(){
                    self.current_card.topics.push(topic);
                    //self.topics.push("".to_string());
                //}
            },
            Message::UpdateTopic(content) => {
                self.current_topic = content;
            },
            Message::SelectTopic(sent_topic) => {
                if let Some(topic) = self.current_card.topics.iter_mut().find(|topic| topic.id == sent_topic.id) {
                    let mut final_color = Color::WHITE;
            
                    enum Op {
                        Push,
                        Retain,
                    }
                    let mut op = None;
            
                    if sent_topic.color.unwrap() == Color::WHITE {
                        final_color = Color::BLACK;
                        op = Some(Op::Retain);
                    } else {
                        final_color = Color::WHITE;
                        op = Some(Op::Push);
                    }
            
                    *topic = Topic {
                        content: topic.content.clone(),
                        color: Some(final_color),
                        id: topic.id,
                    };
            
                    if let Some(op) = op {
                        match op {
                            Op::Retain => self.current_card.topics.retain(|v| v.id != sent_topic.id),
                        }
                    }
                }
            }
            
            Message::AwnserChanged(content) => {
                self.current_card.awnser = content;
            },
            Message::ExpandQuestions => {
                self.expand_questions =  !self.expand_questions;
            },
            Message::ExpandAwnsers => {
                self.expand_awnsers =  !self.expand_awnsers;
            },
            Message::Error => todo!(),
            Message::SelectTest(sent_topic) => {
                if let Some(topic) = self.test.iter_mut().find(|topic| topic.id == sent_topic.id) {
                    let mut final_color = Color::WHITE;
            
                    enum Op {
                        Retain,
                    }
                    let mut op = None;
            
                    if sent_topic.color.unwrap() == Color::WHITE {
                        final_color = Color::BLACK;
                        op = Some(Op::Retain);
                    } else {
                        final_color = Color::WHITE;
                    }
            
                    *topic = Topic {
                        content: topic.content.clone(),
                        color: Some(final_color),
                        id: topic.id,
                    };
            
                    if let Some(op) = op {
                        match op {
                            Op::Retain => self.test.retain(|v| v.id != sent_topic.id),
                        }
                    }
                }
            }
        }
    }
    fn view(&self) -> Container<Message> { 
        let rect: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 1000.0).style(Color::WHITE).into();
        let rect2: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 1000.0).style(Color::WHITE).into();
        let button_style = iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgb8(43, 43, 43))), 
            ..Default::default()
        };

        // if self.current_card.bg_color.is_none(){
        //     self.current_card.bg_color = Some(Background::Color(Color::from_rgb8(43, 43, 43)))
        // }
    
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
        let mut awnser_column: Vec<Element<Message, Theme, Renderer>> = vec![
            Button::new("Expand awnsers").on_press(Message::ExpandAwnsers).into()
        ];
        if self.expand_awnsers {
            awnser_column.push(Text::new(self.current_card.awnser.clone()).into());
        }
        let mut question_column = vec![
            Button::new("Expand questions").on_press(Message::ExpandQuestions).into()
        ];
        if self.expand_questions {
            question_column.push(Text::new(self.current_card.question.clone()).into());
        }
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
                            Button::new("Configure").on_press(Message::Configure),
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
                                        Button::new("Topic")
                                            .style(|_theme: &Theme, _status| {
                                                iced::widget::button::Style {
                                                    background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                    text_color: Color::from_rgb8(255, 255, 255),
                                                    ..Default::default()
                                                }
                                            })
                                            .on_press(Message::Topics),
                                            
                                
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
                                    "1",
                                    Column::new()
                                    .push(
                                        Column::new().push(
                                            Column::with_children(question_column)
                                        ).push(
                                            Space::new(0.0, 10.0)
                                        ).push(
                                            Column::new().push(
                                              Column::with_children(awnser_column)  
                                            
                                            )
                                        )
                                    )
                                    .push(Space::new(0.0, 50))
                                )
                                .foot(Row::with_children(
                                    self.current_card.topics
                                        .iter()
                                        .flat_map(|topic| {
                                            vec![
                                                Text::new(topic.content.clone()).into(), 
                                                Space::new(5, Length::Shrink).into(),
                                            ]
                                        })
                                        .collect::<Vec<_>>(),
                                ))
                                                         
                                .style(|_theme: &Theme, _status|  style::card::Style {
                                    head_background: self.current_card.bg_color.unwrap_or(Background::Color(Color::from_rgb8(255, 0, 0))),
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
                container(
                    modal(
                        main_container,
                        container(stack![
                            background_rect,
                            column!(
                                topic_scrollbar(self),
                                container(Button::new("Exit").on_press(Message::NoPopup)).center_x(Length::Fill)
                            )
                        ]),
                        Message::None
                    )
                )
            },
            Popups::Configure => {
                container(
                    modal(
                        main_container,
                        container(stack![
                            background_rect,
                            column!(
                                topic_scrollbar(self),
                                container(Button::new("Exit").on_press(Message::NoPopup)).center_x(Length::Fill)
                            )
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
                            stack![
                                background_rect,
                                column!(
                                    container(
                                        "Test"
                                    )
                                    .padding(20)
                                    .center_x(Length::Fill)
                                    ,
                                    Space::new(0.0, 20.0),
                                    // Pin::new(
                                        container(
                                            text_input("Type your question here..", &self.current_card.question)
                                                .on_input(Message::QuestionChanged)
                                                
                                        )
                                        .center_x(Length::Fill)
                                        ,
                                    // )           
                                    // .x(280)
                                    // .y(150),
                                    Space::new(0.0, 20.0),
                                    container(
                                        text_input("Type your awnser here..", &self.current_card.awnser)
                                            .on_input(Message::AwnserChanged)
                                            
                                    )
                                    .center_x(Length::Fill),
                                    Space::new(0.0, 20.0),
                                    container(
                                        row!(
                                            Button::new("Exit")
                                            .on_press(Message::NoPopup),
                                            // Button::new("Submit")
                                            // .on_press(Message::SubmitText(self.current_card))
                                        )
                                    )
                                    .center_x(Length::Fill)
                                )
                            ]
                        ,
                        Message::None
                    )
                )
            },
            Popups::Topics => {
        
                container(
                    modal(
                        main_container,
                        stack![
                            background_rect,
                            // row!(
                            // Space::new(200.0, 0.0),
                            // container(
                            //row!(
                            column!(
                                row!(
                                //   Space::new(100.0, 0.0),
                                    row!(
                                        Space::new(100.0, 0.0),
                                        column!(
                                            Space::new(0.0, 80),
                                            container(
                                                column!(
                                                Button::new("Submit").on_press(Message::SubmitTopic(Topic { content: self.current_topic.clone(), color: Some(Color::BLACK), id: self.current_card.topics.len() as u32})),
                                                text_input("Put text here",  &self.current_topic)
                                                .on_input(Message::UpdateTopic),
                                                )
                                            )
                                            // .center_x(Length::Fill)
                                            // .center_y(Length::Fill)
                                            .height(150.0)
                                            .width(120.0)
                                            .padding(10.0)
                                            .style(move |_: &iced::Theme| iced::widget::container::Style {
                                                background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                ..Default::default()
                                            }),
                                        ),
                                    ),
                                    // Space::new(10.0, 0.0),
                                    //Pin::new(
                                    topic_scrollbar(self).center_y(Length::Fill),
                                            //Space::new(0, 50)
                                    //).y(0),
                                    Space::new(60.0, 0.0),
                            ),
                            //Space::new(0.0, 0.0),
                            container(
                                Button::new("Exit").on_press(Message::NoPopup)
                            ).center_x(Length::Fill)
                        ),
                       // )
                       // ).max_width(100),
                       // )
                        ],
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

fn topic_scrollbar(app: &App) -> Container<'static, Message> {
    let mut topic_list: Vec<Element<'_, Message, Theme, Renderer>> = vec![];

    let item_arr = match app.show_modal {
        Popups::Configure => {
            app.current_card.topics.clone()
        }
        Popups::Topics => {
            app.current_card.topics.clone()
        }
        Popups::Test => {
            app.test.clone()
        }
        _ => {
            vec![]
        }
    };
    for (id, topic) in item_arr.iter().enumerate() {
        let topic_background: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 80.0)
            .style(topic.color.unwrap_or(Color::from_rgb8(0, 0, 0)))
            .into();
        let button_background = Some(Background::Color(topic.color.unwrap_or(Color::BLACK)));
        let mut button_text_color = Color::WHITE;
        if topic.color.unwrap() == Color::WHITE {
            button_text_color = Color::BLACK
        } else {
            button_text_color = Color::WHITE
        }
    
        topic_list.push(
            column!(
                Space::new(0.0, 10.0),
                stack![
                    topic_background,
                    container(
                        container(
                            Button::new(Text::new(topic.content.clone()))
                            .style(move |_theme: &Theme, _status| {
                                iced::widget::button::Style {
                                    background: button_background,
                                    text_color: button_text_color,
                                    ..Default::default()
                                }
                            })
                            .on_press(
                                match app.show_modal {
                                    Popups::Configure => {
                                        Message::SelectTest(topic.clone())
                                    }
                                    Popups::Topics => {
                                        Message::SelectTopic(topic.clone())
                                    }
                                    _ => {
                                        Message::None
                                    }
                                }
                                // if true {
                                //     Message::SelectTopic(topic.clone())
                                // } else {
                                //     Message::SelectTopic(topic.clone())
                                // }
                            )
                        )
                        .center(Length::Fill)
                    )
                ],
            )
            .into(),
        );
    
        topic_list.push(Space::new(20.0, 0.0).into());
    }
    
    container(
        container(
            Scrollable::new(
            Row::with_children(
                topic_list
            )
            )
            .height(100.0)
            .width(300.0)
            .style(|_theme: &Theme, _status|  scrollable::Style {
                container: iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgb8(43, 43, 43))),
                    ..Default::default()
                }, 
                vertical_rail: Rail { 
                    background: Some(Background::Color(Color::BLACK)), border: Border { ..Default::default() }, scroller: Scroller { color: Color::WHITE, border: Border { ..Default::default() } }
                },
                horizontal_rail: Rail { 
                    background: Some(Background::Color(Color::from_rgb8(43, 43, 43))), border: Border { ..Default::default() }, scroller: Scroller { color: Color::WHITE, border: Border { ..Default::default() } }
                },
                gap:  Some(Background::Color(Color::BLACK))
            })
            //.spacing(100.0)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new()
            ))
        )
        .style(move |_: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
            ..Default::default()
        })
        .center(Length::Fill)
        .height(150)
        .width(350)
    )
    .center_x(Length::Fill)
    .padding(20.0) 
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