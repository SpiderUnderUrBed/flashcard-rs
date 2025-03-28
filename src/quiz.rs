use iced::{advanced::graphics::futures::backend::default, Background};
use slotmap::{new_key_type, SlotMap};
use std::collections::VecDeque;

// Create key types for topics and flashcards.
new_key_type! {
    pub struct TopicKey;
    pub struct FlashcardKey;
}

// Each topic is assigned a tag. Topics with the Quiz tag will be used for the final quiz,
// while topics with the Configure tag appear in the selection list.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum TopicTag {
    Quiz,
    Configure,
    #[default]
    Default,
    None,
}

// A topic contains some text, a tag, an enabled flag (which might be used to indicate selection)
// and a list of flashcards (by key) that are associated with this topic.
#[derive(Clone, Default, Debug)]
pub struct Topic {
    pub content: String,
    pub topic_tag: TopicTag,
    pub enabled: bool,
    pub qna: Vec<FlashcardKey>,
}

// A flashcard has a background color, a question, an answer, an id and a list of topics (by key)
// to which it belongs.
#[derive(Default, Debug, Clone)]
pub struct Flashcard {
    pub bg_color: Option<Background>,
    pub question: String,
    pub answer: String,
    pub id: u32,
    pub topics: Vec<TopicKey>,
}

// The study session holds all flashcards and topics. It also holds some state about the currently
// selected topics or flashcard as well as a staging field for a new topic.
#[derive(Default, Debug, Clone)]
pub struct Study {
    pub cards: SlotMap<FlashcardKey, Flashcard>,
    pub topics: SlotMap<TopicKey, Topic>,
    pub current_topics: Vec<TopicKey>,
    pub current_topic: Option<TopicKey>,
    pub current_card: Option<FlashcardKey>,
    pub staging_topic: String,
}

// The Quiz holds the flashcards (by key) that are to be quizzed and a queue of Q&A (converted from flashcards).
#[derive(Default, Debug, Clone)]
pub struct Quiz {
    // Keys of flashcards that have been selected for the quiz.
    pub cards: Vec<FlashcardKey>,
    // Queue of questions built from the selected flashcards.
    pub qna_queue: VecDeque<Question>,
}

// A question in the quiz. In this simple implementation it directly mirrors the flashcard fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub id: u32,
}

impl Quiz {
    /// Starts the quiz by scanning the study session for flashcards that qualify.
    ///
    /// A flashcard qualifies if it is associated with at least one topic that is enabled and has the Quiz tag.
    pub fn start_quiz(&mut self, study_session: &Study) {
        self.cards.clear();
        self.qna_queue.clear();
        for (card_key, card) in study_session.cards.iter() {
            // Check if any topic linked to the card qualifies.
            let qualifies = card.topics.iter().any(|&topic_key| {
                if let Some(topic) = study_session.topics.get(topic_key) {
                    topic.enabled && topic.topic_tag == TopicTag::Quiz
                } else {
                    false
                }
            });
            if qualifies {
                self.cards.push(card_key);
                let question = Question {
                    question: card.question.clone(),
                    answer: card.answer.clone(),
                    id: card.id,
                };
                self.qna_queue.push_back(question);
            }
        }
        println!("Quiz started with {} questions.", self.qna_queue.len());
    }

    /// Returns the current quiz layout (the list of questions).
    pub fn get_layout(&self) -> VecDeque<Question> {
        println!("Current Q&A layout: {:?}", self.qna_queue);
        self.qna_queue.clone()
    }

    /// Submits a new card to the quiz.
    ///
    /// The card is only added if it qualifies (i.e. it has at least one associated enabled topic tagged as Quiz).
    /// The caller must supply the flashcard key from the study session.
    pub fn submit_card_to_quiz(
        &mut self,
        card: Flashcard,
        card_key: FlashcardKey,
        study_session: &Study,
    ) {
        let qualifies = card.topics.iter().any(|&topic_key| {
            if let Some(topic) = study_session.topics.get(topic_key) {
                topic.enabled && topic.topic_tag == TopicTag::Quiz
            } else {
                false
            }
        });
        if qualifies {
            self.cards.push(card_key);
            let question = Question {
                question: card.question.clone(),
                answer: card.answer.clone(),
                id: card.id,
            };
            self.qna_queue.push_back(question);
            println!("Card {} submitted to quiz.", card.id);
        } else {
            println!("Card {} not eligible for quiz.", card.id);
        }
    }

    /// Called when the user selects (or toggles) a topic to be part of the quiz.
    ///
    /// In this simple implementation the method toggles the topic’s enabled flag.
    /// Topics that are enabled and marked with the Quiz tag will contribute their flashcards to the quiz.
    pub fn select_topic_to_quiz(&mut self, study_session: &mut Study, topic: TopicKey) {
        if let Some(topic_entry) = study_session.topics.get_mut(topic) {
            topic_entry.enabled = !topic_entry.enabled;
            println!(
                "Topic '{}' is now {} for quiz.",
                topic_entry.content,
                if topic_entry.enabled { "enabled" } else { "disabled" }
            );
        }
    }

    /// Sets the staging topic.
    ///
    /// This is useful when editing or creating a new topic.
    pub fn set_topic(&mut self, study_session: &mut Study, content: String) {
        study_session.staging_topic = content;
        println!("Staging topic set to '{}'.", study_session.staging_topic);
    }

    /// Updates a topic’s content.
    pub fn update_topic(&mut self, study_session: &mut Study) {
        if let Some(current_key) = study_session.current_card {
            if let Some(card) = study_session.cards.get(current_key) {
                let new_qna = Question {
                    question: card.question.clone(),
                    answer: card.answer.clone(),
                    id: card.id,
                };
                // Iterate over topics in the study session and update those associated with the card.
                for topic in study_session.topics.values_mut() {
                    if topic.qna.contains(&current_key) {
                        // For example, here you might log or update some data.
                        println!(
                            "Updating topic '{}' with new Q&A for card {}",
                            topic.content, card.id
                        );
                        // (You could update the topic here if needed.)
                    }
                }
            }
        }
    }

    /// Submits a new topic to the study session’s list and returns its key.
    pub fn submit_topic_to_list(&self, study_session: &mut Study, topic: Topic) -> TopicKey {
        let key = study_session.topics.insert(topic);
        println!("Topic added with key.");
        key
    }

    /// Processes the answer to the current question.
    ///
    /// If the answer is correct (ignoring case and surrounding whitespace) the question is removed from the queue.
    /// Otherwise the question is requeued at the back.
    pub fn answer_current(&mut self, user_answer: String) {
        if let Some(current_question) = self.qna_queue.front() {
            if current_question.answer.trim().eq_ignore_ascii_case(user_answer.trim()) {
                println!("Correct answer for question {}!", current_question.id);
                self.qna_queue.pop_front();
            } else {
                println!("Incorrect answer. Try again.");
                // Optionally requeue the question:
                let question = self.qna_queue.pop_front().unwrap();
                self.qna_queue.push_back(question);
            }
        } else {
            println!("No current question.");
        }
    }

    /// Ends the quiz by clearing the question queue.
    pub fn end_quiz(&mut self) {
        self.qna_queue.clear();
        println!("Quiz ended!");
    }

    /// This helper toggles a topic on the current card.
    ///
    /// When called, if the topic is already enabled on the current card it will be removed;
    /// otherwise it is added.
    pub fn select_topic_for_card(&self, study_session: &mut Study, topic_key: TopicKey) {
        if let Some(topic) = study_session.topics.get_mut(topic_key) {
            if topic.enabled {
                if let Some(current_key) = study_session.current_card {
                    study_session
                        .cards
                        .get_mut(current_key)
                        .unwrap()
                        .topics
                        .retain(|&t| t != topic_key);
                }
                println!("Topic deselected: {}", topic.content);
            } else {
                if let Some(current_key) = study_session.current_card {
                    study_session.cards.get_mut(current_key).unwrap().topics.push(topic_key);
                }
                println!("Topic selected: {}", topic.content);
            }
            topic.enabled = !topic.enabled;
        }
    }

    pub fn change_namespace(){

    }
    pub fn operation_for_namespace(){
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple app structure to help test the quiz functionality.
    struct App {
        cards: SlotMap<FlashcardKey, Flashcard>,
        topics: SlotMap<TopicKey, Topic>,
        quiz: Quiz,
        staging_topic: String,
    }

    #[test]
    fn test_quiz_flow() {
        // Create an empty study session.
        let mut study = Study::default();

        // Insert some topics.
        let mut quiz_topic = Topic {
            content: "Math".to_string(),
            topic_tag: TopicTag::Quiz,
            enabled: false,
            qna: Vec::new(),
        };
        let quiz_topic_key = study.topics.insert(quiz_topic);

        let configure_topic = Topic {
            content: "History".to_string(),
            topic_tag: TopicTag::Configure,
            enabled: false,
            qna: Vec::new(),
        };
        let _configure_topic_key = study.topics.insert(configure_topic);

        // Create a flashcard that belongs to the Math topic.
        let flashcard = Flashcard {
            bg_color: None,
            question: "What is 2+2?".to_string(),
            answer: "4".to_string(),
            id: 1,
            topics: vec![quiz_topic_key],
        };
        let flashcard_key = study.cards.insert(flashcard);

        // Enable the Math topic (so it qualifies for the quiz).
        if let Some(topic) = study.topics.get_mut(quiz_topic_key) {
            topic.enabled = true;
        }

        // Create a quiz and start it using the study session.
        let mut quiz = Quiz::default();
        quiz.start_quiz(&study);

        // Check that the quiz has one question.
        assert_eq!(quiz.qna_queue.len(), 1);

        // Simulate answering the question incorrectly.
        quiz.answer_current("3".to_string());
        // The question should have been requeued.
        assert_eq!(quiz.qna_queue.len(), 1);

        // Now answer correctly.
        quiz.answer_current("4".to_string());
        // Now the question queue should be empty.
        assert_eq!(quiz.qna_queue.len(), 0);

        // End the quiz.
        quiz.end_quiz();
    }
}
