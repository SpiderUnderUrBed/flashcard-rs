use slotmap::{new_key_type, SlotMap};
use std::collections::VecDeque;
//use strum::IntoEnumIterator;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

// Creates two keys with the library slotmap
// TopicKey will be how you access any topic within the Topics slotmap
// FlashcardKey does the same for flashcards, the philosophy is that 
// All topics and all cards are apart of two main vectors, and their tags or vectors with their key's
// determine their state
new_key_type! {
    pub struct TopicKey;
    pub struct FlashcardKey;
}

// Each topic is assigned a tag. Topics with the Quiz tag will be used for the final quiz,
// while topics with the Configure tag appear in the selection list.
#[derive(Clone, Default, EnumIter, Display)]
// #[repr(i32)]
pub enum TopicTag {
    Quiz = 0,
    Configure = 1,
    #[default] 
    Default = 2,
    None = 3,
}

// A topic contains some text, a tag, an enabled flag (which might be used to indicate selection)
// and a list of flashcards (by key) that are associated with this topic.
#[derive(Clone, Default)]
pub struct Topic {
    pub content: String,
    pub topic_tag: TopicTag,
    pub enabled: bool,
    pub qna: Vec<FlashcardKey>,
}

// A flashcard has a question, a awnser, and a topic
// that it belongs too
#[derive(Clone, Default, Debug)]
pub struct Flashcard {
    pub question: String,
    pub awnser: String,
    pub topics: Vec<TopicKey>,
}

// The Quiz holds the flashcards (by key) that are to be quized
// and a queue of Q&A (converted from flashcards)
#[derive(Default, Debug, Clone)]
pub struct Quiz {
    // Keys of the flashcads that have been selected for the quiz.
    pub cards: Vec<FlashcardKey>,
    //SlotMap<FlashcardKey, Flashcard>,
    // Queue of questions built from the selected flashcards.
    pub qna_queue: VecDeque<Question>,
}

// A question in the quiz. In this simple implimnetation it nearly
// completely mirrors the Flashcard struct
#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub id: u32,
}

impl Quiz {
    // TODO

    // pub fn start_quiz(&mut self) {
    //     println!("Quiz started!");
    // }

    // pub fn get_layout(&self) -> VecDeque<Question> {
    //     println!("Current Q&A layout: {:?}", self.qna_queue);
    //     self.qna_queue.clone()
    // }

    // pub fn submit_card_to_quiz(&mut self, card: FlashcardKey) {
    //     self.cards.push(card);
    //     println!("Card submitted to quiz.");
    // }

    // pub fn select_topic_to_quiz(
    //     &mut self,
    //     topic: &Topic,
    //     qna_collection: &SlotMap<FlashcardKey, Question>,
    // ) {
    //     self.qna_queue = topic
    //         .qna
    //         .iter()
    //         .filter_map(|&qna_key| qna_collection.get(qna_key).cloned())
    //         .collect();
    //     println!("Selected topic for quiz: {}", topic.content);
    // }

    // // fn select_quiz(cardkey: FlashcardKey){
    // // }

    // pub fn end_quiz(&mut self) {
    //     self.qna_queue.clear();
    //     println!("Quiz ended!");
    // }
}
