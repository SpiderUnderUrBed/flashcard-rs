use slotmap::{new_key_type, SlotMap};
use std::collections::VecDeque;

new_key_type! {
    pub struct TopicKey;
    pub struct FlashcardKey;
}
#[derive(Clone, Default, Debug)]
pub struct Topic {
    pub content: String,
    pub enabled: bool,
    pub qna: Vec<FlashcardKey>
    //qna: SlotMap<FlashcardKey, Flashcard>,
}


#[derive(Clone, Default, Debug)]
pub struct Flashcard {
    pub question: String,
    pub awnser: String,
    pub topics: Vec<TopicKey>,
}

#[derive(Default, Debug, Clone)]
pub struct Quiz {
    pub cards: Vec<FlashcardKey>,
    //SlotMap<FlashcardKey, Flashcard>,
    pub qna_queue: VecDeque<Question>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub id: u32,
}

impl Quiz {
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
