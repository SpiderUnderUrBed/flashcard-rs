


use std::collections::{VecDeque};

use iced::{advanced::{graphics::{core::Element, futures::backend::default}, Widget}, alignment, widget::{self, button, center, container, mouse_area, opaque, row, scrollable::{self, Rail, Scroller}, stack, text, text_input, Button, Column, Container, Row, Scrollable, Space, Text}, Alignment, Background, Border, Color, Length, Renderer, Settings, Shadow, Size, Task, Theme};

use iced_aw::{card, color_picker, menu::{self, Item, Menu}, menu_bar, style};
use iced_aw::menu_items;
use iced::widget::column;

use slotmap::SlotMap;
use slotmap::new_key_type;

mod rectangle;
mod pin;

use rectangle::rectangle::Rectangle;
use pin::Pin;
use rand::seq::SliceRandom;


new_key_type! {
    pub struct TopicKey;
}
new_key_type! {
    pub struct QnaKey;
}




pub fn main() -> iced::Result {
    iced::run("Title", App::update, App::view)
}

#[derive(Debug, Default, Clone)]
enum Popups {
    Flashcards,
    Assist,
    Test,
    ColorPicker,
    Text,
    StartTest(VecDeque<QNA>),
    Topics,
    Configure,
    #[default]
    None
}

#[derive(Default)]
struct App {
    show_modal: Popups,
    current_test: test_module::Test,
    current_card: Flashcard,

    test: SlotMap<TopicKey, Topic>,
    topics: SlotMap<TopicKey, Topic>,
    configurable_topics: SlotMap<TopicKey, Topic>,

    qna_collection: SlotMap<QnaKey, QNA>,

    current_topic: Option<TopicKey>,
    staging_topic: String,
    expand_questions: bool,
    expand_awnsers: bool,
}
#[derive(Debug, Clone, Default)]
struct Topic {
    key: Option<TopicKey>,
    content: String,
    color: Option<Color>,
    qna: Vec<QnaKey>,
    
}

#[derive(Default, Debug, Clone)]
struct Flashcard {
    bg_color: Option<Background>,
    header: String,
    footer: String,
    question: String,
    awnser: String,
    id: u32,
    topics: Vec<TopicKey>
}


#[derive(Debug, Clone, PartialEq)]
struct QNA {
    question: String,
    awnser: String,
    id: u32
}

mod test_module {
    use super::{Flashcard, QNA, Topic, TopicKey};
    use std::collections::VecDeque;
    use slotmap::SlotMap;
    use super::QnaKey;

    #[derive(Default, Debug, Clone)]
    pub struct Test {
        cards: Vec<Flashcard>,
        pub qna_queue: VecDeque<QNA>,
    }

    impl Test {
        pub fn start_test(&mut self) {
            println!("Test started!");
        }

        pub fn get_layout(&self) -> VecDeque<QNA> {
            println!("Current Q&A layout: {:?}", self.qna_queue);
            self.qna_queue.clone()
        }

        pub fn submit_card_to_test(&mut self, card: Flashcard) {
            self.cards.push(card);
            println!("Card submitted to test.");
        }
        
        pub fn select_topic_to_test(
            &mut self,
            topic: &Topic,
            qna_collection: &SlotMap<QnaKey, QNA>,
        ) {
            self.qna_queue = topic
                .qna
                .iter()
                .filter_map(|&qna_key| qna_collection.get(qna_key).cloned())
                .collect();
            println!("Selected topic for test: {}", topic.content);
        }

        pub fn end_test(&mut self) {
            self.qna_queue.clear();
            println!("Test ended!");
        }

        

    }

    
    pub fn add_qna_to_topic(
        card: &Flashcard,
        topic: &mut Topic,
        qna_collection: &mut SlotMap<QnaKey, QNA>,
    ) {
        let new_qna = QNA {
            question: card.header.clone(),
            awnser: card.footer.clone(),
            id: card.id,
        };
        let qna_key = qna_collection.insert(new_qna.clone());
        topic.qna.push(qna_key);
        println!("QNA added to topic: {}", topic.content);
    }
    
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
    SubmitCard(Flashcard),
    UpdateTest(VecDeque<QNA>),
    EndTest,
    UpdateTopic,
    SetTopic(String),
    SelectTopic(TopicKey),
    SelectTest(TopicKey),
    StartTest,
    AwnserChanged(String),
    QuestionChanged(String),
    ExpandQuestions,
    ExpandAwnsers,
    Text,
    Topics,    
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
            Message::QuestionChanged(content) => {
                self.current_card.question = content;
                self.update(Message::UpdateTopic);
               
            },
            Message::StartTest => {
                self.current_test.start_test();
                let questions = self.current_test.get_layout();
                self.show_modal = Popups::StartTest(questions);
            },
            
            
            Message::SelectTopic(topic_key) => {
                if let Some(topic) = self.topics.get_mut(topic_key) {
                    
                    let is_selected = topic.color.unwrap_or(Color::BLACK) == Color::WHITE;
            
                    if is_selected {
                        
                        self.current_card.topics.retain(|&t| t != topic_key);
                        topic.color = Some(Color::BLACK);
                        println!("Topic deselected: {}", topic.content);
                    } else {
                        
                        self.current_card.topics.push(topic_key);
                        topic.color = Some(Color::WHITE);
                        println!("Topic selected: {}", topic.content);
                    }
                }
            },
            
            Message::SelectTest(topic_key) => {
                if let Some(topic) = self.configurable_topics.get_mut(topic_key) {
                    println!("Processing topic: {}", topic.content);
                    
                    let is_selected = topic.color.unwrap_or(Color::WHITE) == Color::BLACK;
                    if is_selected {
                        topic.color = Some(Color::WHITE)
                    } else {
                        topic.color = Some(Color::BLACK)
                    }

                    // println!("Updating current test session...");
                    self.current_test.select_topic_to_test(topic, &self.qna_collection);
                    println!("Topic {} processed.", topic.content);
                } else {
                   // println!("Topic key {} not found in configurable_topics.", topic_key);
                }
            }
            
            
            
            Message::SubmitCard(card) => {
                self.current_test.submit_card_to_test(card);
            },
            
            Message::EndTest => {
                self.current_test.end_test();
                self.update(Message::NoPopup);
            },
            
            Message::AwnserChanged(content) => {
                self.current_card.awnser = content;
                self.update(Message::UpdateTopic);
            },
            Message::ExpandQuestions => {
                self.expand_questions =  !self.expand_questions;
            },
            Message::ExpandAwnsers => {
                self.expand_awnsers =  !self.expand_awnsers;
            },
            Message::Error => todo!(),
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
            Message::SubmitTopic(mut topic) => {
                
                
                let _topic_key = self.topics.insert(topic.clone());
                let _conf_topic_key = self.configurable_topics.insert(topic);
            },
            
            Message::SetTopic(content) => {
                
                
                self.staging_topic = content.clone();
                if let Some(topic_key) = self.current_topic {
                    if let Some(topic) = self.topics.get_mut(topic_key) {
                        topic.content = content;
                    }
                }
            },
            
            Message::UpdateTest(qna_queue) => {
                
                
                self.current_test.qna_queue.pop_front();
                self.show_modal = Popups::StartTest(qna_queue);
            },
            
            Message::UpdateTopic => {
                let topics: Vec<&mut Topic> = self.topics.values_mut().collect();
                for mut topic in topics {
                    if topic.qna.pop().is_some() {
                        let new_qna = QNA {
                            question: self.current_card.question.clone(),
                            awnser: self.current_card.awnser.clone(),
                            id: self.current_card.id,
                        };
                        let new_qna_key = self.qna_collection.insert(new_qna);
                        topic.qna.push(new_qna_key);
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
                                        Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                                        Button::new("Submit")
                                            .style(|_theme: &Theme, _status| {
                                                iced::widget::button::Style {
                                                    background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                    text_color: Color::from_rgb8(255, 255, 255),
                                                    ..Default::default()
                                                }
                                            })
                                            .on_press(Message::SubmitCard(self.current_card.clone()))
                                            ,
                            )),
                            Space::new(7.5, 0.0),
                            column!(
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
                                        .filter_map(|topic_key| self.topics.get(*topic_key)) 
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

        match &self.show_modal {
            
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
                                Space::new(0.0, 20.0),
                                container(Button::new("Start test").on_press(Message::StartTest)).center_x(Length::Fill),
                                Space::new(0.0, 20.0),
                                container(Button::new("Exit").on_press(Message::EndTest)).center_x(Length::Fill)
                            )
                        ]),
                        Message::None
                    )
                )
            },
            Popups::StartTest(local_qna) => {
                let mut display_sidecards: Vec<iced::Element<'_, Message>> = vec![];
            
                for qna in local_qna {
                    let id = qna.id;
                    let question = qna.question.clone();
                    let awnser = qna.awnser.clone();
                    
                        let element: iced::Element<'_, Message> = Container::new(Text::new(question.clone()))
                            .width(110)
                            .height(50)
                            .style(|_: &iced::Theme| iced::widget::container::Style {
                                background: Some(iced::Background::Color(iced::Color::WHITE)),
                                ..Default::default()
                            })
                            .into();
            
                        display_sidecards.push(element);
                   
                }
            
                let content: iced::Element<'_, Message> = if let Some(qna) = self.current_test.qna_queue.front() {
                    let question = qna.question.clone();
                    Text::new(question.clone()).into()
                } else {
                    Text::new("Finished").into()
                };
            
                let main_column = column!(
                    container(
                        row!(
                            Space::new(5, 0),
                            container(
                                row!(
                                    Space::new(20.0, 0.0),
                                    Scrollable::new(
                                        Column::with_children(display_sidecards) 
                                    )
                                )
                            )
                            .width(150)
                            .height(250)
                            .style(|_: &iced::Theme| iced::widget::container::Style {
                                background: Some(iced::Background::Color(iced::Color::BLACK)),
                                ..Default::default()
                            }),
                            Space::new(5, 0),
                            container(
                                container(
                                    column!(
                                        content,
                                        Button::new("Next").on_press(Message::UpdateTest(local_qna.clone())) 
                                    )
                                )
                                .width(250)
                                .height(150)
                                .style(|_: &iced::Theme| iced::widget::container::Style {
                                    background: Some(iced::Background::Color(iced::Color::WHITE)),
                                    ..Default::default()
                                }),
                            )
                            .center(Length::Fill)
                            .width(500)
                            .height(250)
                            .style(|_: &iced::Theme| iced::widget::container::Style {
                                background: Some(iced::Background::Color(iced::Color::BLACK)),
                                ..Default::default()
                            })
                        )
                    ),
                    container(Button::new("Exit").on_press(Message::NoPopup)).center_x(Length::Fill)
                );
            
                container(modal(main_container, main_column, Message::None))
            }
            
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
                                    
                                        color_picker(
                                            true,
                                            Color::BLACK,
                                            Button::new(Text::new("Set Color")).on_press(Message::ChooseColor),
                                            Message::CancelColor,
                                            Message::SubmitColor,
                                        ),
                                    
                                
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
                                    
                                        container(
                                            text_input("Type your question here..", &self.current_card.question)
                                                .on_input(Message::QuestionChanged)
                                                
                                        )
                                        .center_x(Length::Fill)
                                        ,
                                    
                                    
                                    
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
                            column!(
                                row!(
                                    row!(
                                        Space::new(100.0, 0.0),
                                        column!(
                                            Space::new(0.0, 80),
                                            container(
                                                column!(
                                                    
                                                    if let Some(topic_key) = self.current_topic {
                                                        if let Some(topic) = self.topics.get(topic_key) {
                                                            column!(
                                                                Button::new("Submit")
                                                                    .on_press(Message::SubmitTopic(Topic {content:topic.content.clone(),color:Some(Color::BLACK),qna:topic.qna.clone(), key: None })),
                                                                text_input("Put text here", &topic.content)
                                                                    .on_input(Message::SetTopic),
                                                            )
                                                        } else {
                                                            column!(
                                                                Text::new("Topic not found")
                                                            )
                                                        }
                                                    } else {
    
                                                        column!(
                                                            text_input("Enter new topic", &self.staging_topic)
                                                                .on_input(Message::SetTopic),
                                                            Button::new("Submit")
                                                                .on_press(Message::SubmitTopic(
                                                                    Topic {content:self.staging_topic.clone(),color:Some(Color::BLACK),qna:vec![], key: None }
                                                            )),
                                                        )
                                                    }
                                                )
                                            )
                                            .height(150.0)
                                            .width(120.0)
                                            .padding(10.0)
                                            .style(move |_: &iced::Theme| iced::widget::container::Style {
                                                background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                                                ..Default::default()
                                            }),
                                        ),
                                    ),
                                    topic_scrollbar(self).center_y(Length::Fill),
                                    Space::new(60.0, 0.0),
                                ),
                                container(
                                    Button::new("Exit").on_press(Message::NoPopup)
                                )
                                .center_x(Length::Fill)
                            ),
                        ],
                        Message::None,
                    )
                )
            }
            
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
        let label = self.main_label; 
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
    
    let item_arr: SlotMap<_, _> =  match app.show_modal {
        Popups::Topics => {
            
            app.topics.clone()
        }
        Popups::Configure => {
            
            app.configurable_topics.clone()
        }
        Popups::Test => {
            app.test.clone()
            
        }
        _ => {
            let final_slot: SlotMap<TopicKey, Topic> = SlotMap::with_key();
            final_slot
        }
    };
    for (id, topic) in item_arr.iter() {
        let final_color = topic.color.unwrap_or(Color::from_rgb8(0, 0, 0));
        let topic_background: Element<'_, Message, Theme, Renderer> = Rectangle::new(100.0, 80.0)
            .style(final_color)
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
                                        
                                        
                                        
                                        
                                        Message::SelectTest(id)
                                    }
                                    Popups::Topics => {
                                        Message::SelectTopic(id)
                                    }
                                    _ => {
                                        Message::None
                                    }
                                }
                                
                                
                                
                                
                                
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