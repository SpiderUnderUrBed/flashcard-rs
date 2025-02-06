use super::QnaKey;
use super::{Flashcard, Topic, QNA};
use slotmap::SlotMap;
use std::collections::VecDeque;

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

    pub fn select_topic_to_test(&mut self, topic: &Topic, qna_collection: &SlotMap<QnaKey, QNA>) {
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
        answer: card.footer.clone(),
        id: card.id,
    };
    let qna_key = qna_collection.insert(new_qna.clone());
    topic.qna.push(qna_key);
    println!("QNA added to topic: {}", topic.content);
}
