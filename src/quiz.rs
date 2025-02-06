use super::QuestionKey;
use super::{Flashcard, Question, Topic};
use slotmap::SlotMap;
use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct Quiz {
    cards: Vec<Flashcard>,
    pub qna_queue: VecDeque<Question>,
}

impl Quiz {
    pub fn start_quiz(&mut self) {
        println!("Quiz started!");
    }

    pub fn get_layout(&self) -> VecDeque<Question> {
        println!("Current Q&A layout: {:?}", self.qna_queue);
        self.qna_queue.clone()
    }

    pub fn submit_card_to_quiz(&mut self, card: Flashcard) {
        self.cards.push(card);
        println!("Card submitted to quiz.");
    }

    pub fn select_topic_to_quiz(
        &mut self,
        topic: &Topic,
        qna_collection: &SlotMap<QuestionKey, Question>,
    ) {
        self.qna_queue = topic
            .qna
            .iter()
            .filter_map(|&qna_key| qna_collection.get(qna_key).cloned())
            .collect();
        println!("Selected topic for quiz: {}", topic.content);
    }

    pub fn end_quiz(&mut self) {
        self.qna_queue.clear();
        println!("Quiz ended!");
    }
}
