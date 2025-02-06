use std::collections::VecDeque;

use iced::{
    advanced::graphics::core::Element,
    alignment,
    widget::{
        button, center, column, container, mouse_area, opaque, row,
        scrollable::{self, Rail, Scroller},
        stack, text_input, Button, Column, Container, Row, Scrollable, Space, Text,
    },
    Alignment, Background, Border, Color,
    Length::{self}, Renderer, Theme,
};

use iced_aw::{card, color_picker, style};

use slotmap::new_key_type;
use slotmap::SlotMap;

mod pin;
mod quiz;
mod rectangle;

use pin::Pin;
use rectangle::RoundedRectangle;

new_key_type! {
    pub struct TopicKey;
    pub struct QuestionKey;
}

pub fn main() -> iced::Result {
    iced::run("Title", App::update, App::view)
}

#[derive(Debug, Default, Clone)]
enum Popups {
    Flashcards,
    Quiz,
    ColorPicker,
    Text,
    StartQuiz(VecDeque<Question>),
    Topics,
    Configure,
    #[default]
    None,
}

#[derive(Default)]
struct App {
    current_popup: Popups,
    current_quiz: quiz::Quiz,
    current_card: Flashcard,

    quiz: SlotMap<TopicKey, Topic>,
    topics: SlotMap<TopicKey, Topic>,
    configurable_topics: SlotMap<TopicKey, Topic>,

    qna_collection: SlotMap<QuestionKey, Question>,

    current_topic: Option<TopicKey>,
    staging_topic: String,
    expand_questions: bool,
    expand_answers: bool,
}

#[derive(Debug, Clone, Default)]
struct Topic {
    content: String,
    enabled: bool,
    qna: Vec<QuestionKey>,
}

#[derive(Default, Debug, Clone)]
struct Flashcard {
    bg_color: Option<Background>,
    question: String,
    answer: String,
    id: u32,
    topics: Vec<TopicKey>,
}

#[derive(Debug, Clone, PartialEq)]
struct Question {
    question: String,
    answer: String,
    id: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Flashcards,
    Quiz,
    Configure,
    NoPopup,
    ColorPicker,
    CancelColor,
    ChooseColor,
    SubmitColor(Color),
    SubmitTopic(Topic),
    SubmitCard(Flashcard),
    UpdateQuiz(VecDeque<Question>),
    EndQuiz,
    UpdateTopic,
    SetTopic(String),
    SelectTopic(TopicKey),
    SelectQuiz(TopicKey),
    StartQuiz,
    AnswerChanged(String),
    QuestionChanged(String),
    ExpandQuestions,
    ExpandAnswers,
    Text,
    Topics,
    None,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::QuestionChanged(content) => {
                self.current_card.question = content;
                self.update(Message::UpdateTopic);
            }
            Message::StartQuiz => {
                self.current_quiz.start_quiz();
                let questions = self.current_quiz.get_layout();
                self.current_popup = Popups::StartQuiz(questions);
            }

            Message::SelectTopic(topic_key) => {
                if let Some(topic) = self.topics.get_mut(topic_key) {
                    if topic.enabled {
                        self.current_card.topics.retain(|&t| t != topic_key);
                        println!("Topic deselected: {}", topic.content);
                    } else {
                        self.current_card.topics.push(topic_key);
                        println!("Topic selected: {}", topic.content);
                    }
                    topic.enabled = !topic.enabled;
                }
            }

            Message::SelectQuiz(topic_key) => {
                if let Some(topic) = self.configurable_topics.get_mut(topic_key) {
                    println!("Processing topic: {}", topic.content);

                    topic.enabled = !topic.enabled;
                    // let is_selected = topic.enabled;
                    // if is_selected {
                    //     topic.color = Some(Color::WHITE)
                    // } else {
                    //     topic.color = Some(Color::BLACK)
                    // }

                    // println!("Updating current quiz session...");
                    self.current_quiz
                        .select_topic_to_quiz(topic, &self.qna_collection);
                    println!("Topic {} processed.", topic.content);
                } else {
                    println!(
                        "Topic key {:?} not found in configurable_topics.",
                        topic_key
                    );
                }
            }

            Message::SubmitCard(card) => {
                self.current_quiz.submit_card_to_quiz(card);
            }

            Message::EndQuiz => {
                self.current_quiz.end_quiz();
                self.update(Message::NoPopup);
            }

            Message::AnswerChanged(content) => {
                self.current_card.answer = content;
                self.update(Message::UpdateTopic);
            }
            Message::ExpandQuestions => self.expand_questions = !self.expand_questions,
            Message::ExpandAnswers => self.expand_answers = !self.expand_answers,

            Message::SubmitColor(color) => {
                self.current_card = Flashcard {
                    bg_color: Some(Background::Color(color)),
                    question: self.current_card.question.clone(),
                    answer: self.current_card.answer.clone(),
                    topics: self.current_card.topics.clone(),

                    ..Default::default()
                }
            }

            // Popups
            Message::Text => self.current_popup = Popups::Text,
            Message::Topics => self.current_popup = Popups::Topics,
            Message::Quiz => self.current_popup = Popups::Quiz,
            Message::Configure => self.current_popup = Popups::Configure,
            Message::NoPopup => self.current_popup = Popups::None,
            Message::ColorPicker => self.current_popup = Popups::ColorPicker,
            Message::Flashcards => self.current_popup = Popups::Flashcards,

            // Miscellaneous
            Message::None | Message::CancelColor | Message::ChooseColor => {}

            Message::SubmitTopic(topic) => {
                self.topics.insert(topic.clone());
                self.configurable_topics.insert(topic);
            }

            Message::SetTopic(content) => {
                self.staging_topic = content.clone();
                if let Some(topic_key) = self.current_topic {
                    if let Some(topic) = self.topics.get_mut(topic_key) {
                        topic.content = content;
                    }
                }
            }

            Message::UpdateQuiz(qna_queue) => {
                self.current_quiz.qna_queue.pop_front();
                self.current_popup = Popups::StartQuiz(qna_queue);
            }

            Message::UpdateTopic => {
                let topics = self.topics.values_mut().collect::<Vec<&mut Topic>>();
                for topic in topics {
                    if topic.qna.pop().is_some() {
                        let new_qna = Question {
                            question: self.current_card.question.clone(),
                            answer: self.current_card.answer.clone(),
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
        let main_container = container(self.main_container());
        let background_rect: Element<'_, Message, Theme, Renderer> =
            RoundedRectangle::new(700.0, 340.0)
                .bg_color(Color::from_rgb8(81, 80, 80))
                .border_radius(20.0)
                .into();

        dbg!(&self.current_popup);
        match &self.current_popup {
            Popups::Flashcards => {
                let rect: Element<'_, Message, Theme, Renderer> =
                    RoundedRectangle::new(200.0, 200.0)
                        .bg_color(Color::WHITE)
                        .into();
                container(popup(
                    main_container,
                    container(stack![
                        rect,
                        container(Button::new("Quiz").on_press(Message::NoPopup))
                            .center_x(Length::Fill)
                    ]),
                    Message::None,
                ))
            }
            Popups::None => main_container,
            Popups::Quiz => container(popup(
                main_container,
                container(stack![
                    background_rect,
                    column!(
                        topic_scrollbar(self),
                        Space::new(0.0, 20.0),
                        container(Button::new("Start quiz").on_press(Message::StartQuiz))
                            .center_x(Length::Fill),
                        Space::new(0.0, 20.0),
                        container(Button::new("Exit").on_press(Message::EndQuiz))
                            .center_x(Length::Fill)
                    )
                ]),
                Message::None,
            )),
            Popups::StartQuiz(local_qna) => {
                let display_sidecards = local_qna.iter().map(|qna| {
                    Container::new(Text::new(qna.question.clone()))
                        .width(110)
                        .height(50)
                        .style(|_| container::Style::default().background(Color::WHITE))
                        .into()
                });

                let content = if let Some(qna) = self.current_quiz.qna_queue.front() {
                    Text::new(qna.question.clone())
                } else {
                    Text::new("Finished")
                };

                let main_column = column!(
                    container(row!(
                        Space::new(5, 0),
                        container(row!(
                            Space::new(20.0, 0.0),
                            Scrollable::new(Column::with_children(display_sidecards))
                        ))
                        .width(150)
                        .height(250)
                        .style(|_| container::Style::default().background(Color::BLACK)),
                        Space::new(5, 0),
                        container(
                            container(column!(
                                content,
                                Button::new("Next")
                                    .on_press(Message::UpdateQuiz(local_qna.clone()))
                            ))
                            .width(250)
                            .height(150)
                            .style(|_| container::Style::default().background(Color::WHITE)),
                        )
                        .center(Length::Fill)
                        .width(500)
                        .height(250)
                        .style(|_| container::Style::default().background(Color::BLACK)),
                    )),
                    container(Button::new("Exit").on_press(Message::NoPopup))
                        .center_x(Length::Fill)
                );

                container(popup(main_container, main_column, Message::None))
            }

            Popups::Configure => container(popup(
                main_container,
                container(stack![
                    background_rect,
                    column!(
                        topic_scrollbar(self),
                        container(Button::new("Exit").on_press(Message::NoPopup))
                            .center_x(Length::Fill)
                    )
                ]),
                Message::None,
            )),
            Popups::ColorPicker => container(popup(
                main_container,
                container(stack![
                    background_rect,
                    row!(
                        Pin::new(color_picker(
                            true,
                            Color::BLACK,
                            Button::new(Text::new("Set Color")).on_press(Message::ChooseColor),
                            Message::CancelColor,
                            Message::SubmitColor,
                        ),)
                        .x(280)
                        .y(150),
                        Button::new("Exit").on_press(Message::NoPopup)
                    )
                ]),
                Message::None,
            )),
            Popups::Text => container(popup(
                main_container,
                stack![
                    background_rect,
                    column!(
                        container("Quiz").padding(20).center_x(Length::Fill),
                        Space::new(0.0, 20.0),
                        container(
                            text_input("Type your question here..", &self.current_card.question)
                                .on_input(Message::QuestionChanged)
                        )
                        .center_x(Length::Fill),
                        Space::new(0.0, 20.0),
                        container(
                            text_input("Type your answer here..", &self.current_card.answer)
                                .on_input(Message::AnswerChanged)
                        )
                        .center_x(Length::Fill),
                        Space::new(0.0, 20.0),
                        container(row!(Button::new("Exit").on_press(Message::NoPopup),))
                            .center_x(Length::Fill)
                    )
                ],
                Message::None,
            )),
            Popups::Topics => container(popup(
                main_container,
                stack![
                    background_rect,
                    column!(
                        row!(
                            row!(
                                Space::new(100.0, 0.0),
                                column!(
                                        Space::new(0.0, 80),
                                        container(column!(if let Some(topic_key) =
                                            self.current_topic
                                        {
                                            if let Some(topic) = self.topics.get(topic_key) {
                                                column!(
                                                    Button::new("Submit").on_press(
                                                        Message::SubmitTopic(Topic {
                                                            content: topic.content.clone(),
                                                            enabled: false,
                                                            qna: topic.qna.clone(),
                                                        })
                                                    ),
                                                    text_input("Put text here", &topic.content)
                                                        .on_input(Message::SetTopic),
                                                )
                                            } else {
                                                column!(Text::new("Topic not found"))
                                            }
                                        } else {
                                            column!(
                                                text_input("Enter new topic", &self.staging_topic)
                                                    .on_input(Message::SetTopic),
                                                Button::new("Submit").on_press(
                                                    Message::SubmitTopic(Topic {
                                                        content: self.staging_topic.clone(),
                                                        enabled: false,
                                                        qna: vec![],
                                                    })
                                                ),
                                            )
                                        }))
                                        .height(150.0)
                                        .width(120.0)
                                        .padding(10.0)
                                        .style(|_| container::Style::default()
                                            .background(Color::BLACK)),
                                    ),
                            ),
                            topic_scrollbar(self).center_y(Length::Fill),
                            Space::new(60.0, 0.0),
                        ),
                        container(Button::new("Exit").on_press(Message::NoPopup))
                            .center_x(Length::Fill)
                    ),
                ],
                Message::None,
            )),
        }
    }

    fn main_container(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut answer_column: Vec<Element<Message, Theme, Renderer>> =
            vec![Button::new("Expand answers")
                .on_press(Message::ExpandAnswers)
                .into()];
        if self.expand_answers {
            answer_column.push(Text::new(self.current_card.answer.clone()).into());
        }
        let mut question_column = vec![Button::new("Expand questions")
            .on_press(Message::ExpandQuestions)
            .into()];
        if self.expand_questions {
            question_column.push(Text::new(self.current_card.question.clone()).into());
        }

        let btn_style = button::Style {
            background: Some(Color::BLACK.into()),
            text_color: Color::WHITE,
            ..Default::default()
        };

        row![
            container(
                column![
                    Space::new(0., 0.),
                    Text::new(" Learn\n     &\nExplore")
                        .center()
                        .color(Color::WHITE)
                        .width(Length::Fill),
                    Button::new("Flashcards").on_press(Message::Flashcards),
                    Button::new("Quiz").on_press(Message::Quiz),
                    Button::new("Configure").on_press(Message::Configure),
                ]
                .align_x(alignment::Horizontal::Center)
                .spacing(15.)
            )
            .width(Length::Fixed(110.0))
            .height(Length::Fill)
            .style(|_| container::Style::default().background(Color::from_rgb8(43, 43, 43))),
            Space::new(60.0, 0.0),
            container(row!(
                Space::new(7.5, 0.0),
                container(column!(
                    Button::new("Text")
                        .style(move |_, _| btn_style)
                        .on_press(Message::Text),
                    Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                    Button::new("Topic")
                        .style(move |_, _| btn_style)
                        .on_press(Message::Topics),
                    Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                    Button::new("Color")
                        .style(move |_, _| btn_style)
                        .on_press(Message::ColorPicker),
                    Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                    Button::new("Image").style(move |_, _| btn_style),
                    Space::new(Length::Fixed(0.0), Length::Fixed(5.0)),
                    Button::new("Submit")
                        .style(move |_, _| btn_style)
                        .on_press(Message::SubmitCard(self.current_card.clone())),
                )),
                Space::new(7.5, 0.0),
                column!(card(
                    "1",
                    Column::new()
                        .push(
                            Column::new()
                                .push(Column::with_children(question_column))
                                .push(Space::new(0.0, 10.0))
                                .push(Column::new().push(Column::with_children(answer_column)))
                        )
                        .push(Space::new(0.0, 50))
                )
                .foot(Row::with_children(
                    self.current_card
                        .topics
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
                .style(|_theme: &Theme, _status| style::card::Style {
                    head_background: self
                        .current_card
                        .bg_color
                        .unwrap_or(Color::from_rgb8(255, 0, 0).into()),
                    ..Default::default()
                })
                .width(Length::Fixed(400.0)))
            ))
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(300.0))
            .style(|_| container::Style::default().background(Color::from_rgb8(43, 43, 43)))
            .align_x(Alignment::Center)
        ]
        .into()
    }
}

fn topic_scrollbar(app: &App) -> Container<'static, Message> {
    let mut topic_list = vec![];

    let item_arr: SlotMap<_, _> = match app.current_popup {
        Popups::Topics => app.topics.clone(),
        Popups::Configure => app.configurable_topics.clone(),
        Popups::Quiz => app.quiz.clone(),
        _ => SlotMap::with_key(),
    };
    for (id, topic) in item_arr.iter() {
        let final_color = if topic.enabled {
            Color::WHITE
        } else {
            Color::BLACK
        };
        let topic_background: Element<'_, Message, Theme, Renderer> =
            RoundedRectangle::new(100.0, 80.0)
                .bg_color(final_color)
                .into();

        let button_text_color = if !topic.enabled {
            Color::WHITE
        } else {
            Color::BLACK
        };

        topic_list.push(
            column!(
                Space::new(0.0, 10.0),
                stack![
                    topic_background,
                    container(
                        container(
                            Button::new(Text::new(topic.content.clone()))
                                .style(move |_, _| {
                                    button::Style {
                                        background: Some(final_color.into()),
                                        text_color: button_text_color,
                                        ..Default::default()
                                    }
                                })
                                .on_press(match app.current_popup {
                                    Popups::Configure => Message::SelectQuiz(id),
                                    Popups::Topics => Message::SelectTopic(id),
                                    _ => Message::None,
                                })
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
            Scrollable::new(Row::with_children(topic_list))
                .height(100.0)
                .width(300.0)
                .style(|_, _| scrollable::Style {
                    container: container::Style::default().background(Color::from_rgb8(43, 43, 43)),
                    vertical_rail: Rail {
                        background: Some(Background::Color(Color::BLACK)),
                        border: Border {
                            ..Default::default()
                        },
                        scroller: Scroller {
                            color: Color::WHITE,
                            border: Border {
                                ..Default::default()
                            },
                        },
                    },
                    horizontal_rail: Rail {
                        background: Some(Background::Color(Color::from_rgb8(43, 43, 43))),
                        border: Border::default(),
                        scroller: Scroller {
                            color: Color::WHITE,
                            border: Border {
                                ..Default::default()
                            },
                        },
                    },
                    gap: Some(Background::Color(Color::BLACK)),
                })
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::new(),
                )),
        )
        .style(|_| container::Style::default().background(Color::BLACK))
        .center(Length::Fill)
        .height(150)
        .width(350),
    )
    .center_x(Length::Fill)
    .padding(20.0)
}

fn popup<'a, Message>(
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
            mouse_area(center(opaque(content)).style(|_| Color::BLACK.scale_alpha(0.8).into()))
                .on_press(on_blur)
        )
    ]
    .into()
}
