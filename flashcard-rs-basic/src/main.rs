mod quiz;
use std::{collections::VecDeque, fmt::Error, io::{stdin, stdout, Write}, iter::FilterMap};
use quiz::{Flashcard, FlashcardKey, Quiz, Topic, TopicKey};
use slotmap::{new_key_type, SlotMap};
// use quiz::QuestionKey'

fn ask_question() -> String {
    let mut awnser = String::new();
    stdout().flush().unwrap();
    if let Err(error) = stdin().read_line(&mut awnser) {
        println!("Error: {error}");
    }
    println!("\n");

    awnser.trim().to_string()
}



#[derive(PartialEq, Debug)]
enum Operations {
    Make,
    Edit,
    List,
    Topic,
    Auto,
    Delete,
    Add,
    Test,
    None
}
trait UserInput {
    fn ask_question(&mut self) -> String;
}

struct RealInput;

impl UserInput for RealInput {
    fn ask_question(&mut self) -> String {
        let mut answer = String::new();
        stdout().flush().unwrap();
        if let Err(error) = stdin().read_line(&mut answer) {
            println!("Error: {error}");
        }
        println!("\n");

        answer.trim().to_string()
    }
}

struct App<I: UserInput> {
    input: I,
    cards: SlotMap<FlashcardKey, Flashcard>,
    topics: SlotMap<TopicKey, Topic>,
    current_topics: Vec<TopicKey>,
    quiz: Quiz
}
impl Default for App<RealInput> { 
    fn default() -> Self {
        Self {
            input: RealInput,
            ..Default::default()
        }
    }
}

impl App<RealInput> { 
fn new() -> Self {
    Self {
        input: RealInput,
        cards: SlotMap::with_key(),
        topics: SlotMap::with_key(),
        quiz: Quiz::default(),
        current_topics: vec![],
        
    }
}
}

impl<I: UserInput> App<I> {
    fn operate(&mut self, operation: Operations){
        match operation {
            Operations::Make => {
                println!("What is your question?");
                let question = self.input.ask_question();
    
                println!("What is your awnser?");
                let awnser = self.input.ask_question();
    
                for (_, value) in &self.topics {
                    println!("{}", value.content);
                }
    
                println!("What topics do you want it to be in? (split it from comma)");
                let topic_string = self.input.ask_question();
                let topics: Vec<&str> = topic_string.split(",").collect();
                
                let added_card_key = self.cards.insert(Flashcard { question, awnser, topics: vec![] });
    
                let mut final_topic_keys: Vec<TopicKey> = vec![];
                for listed_topic in topics {
                    for (key, topic) in self.topics.iter_mut() {
                        if topic.content.trim().to_lowercase() == listed_topic.trim().to_lowercase() {
                            topic.qna.push(added_card_key);
                            final_topic_keys.push(key);
                        }
                    }
                }
            
    
                self.cards.get_mut(added_card_key).unwrap().topics.extend(final_topic_keys);
            },
            Operations::Edit => {
                loop {
                    let mut identifyable_cards: Vec<(FlashcardKey, Flashcard)> = vec![];
                    for (i, (key, card)) in self.cards.iter().enumerate() {
                        println!(
                            "\n{}: question: {}, answer: {}",
                            i, card.question, card.awnser
                        );
                        identifyable_cards.push((key, card.clone()));
                    
                    }
                    println!("What card do you want to edit?");
                    let card_to_edit: Result<usize, _> = self.input.ask_question().parse();
                    if card_to_edit.is_err() {
                        println!("Invalid input");
                        continue;
                    }
            
                    
                    let (card_key, card) = &mut identifyable_cards[card_to_edit.unwrap()];
                    
                    println!("Type 1 to edit question");
                    println!("Type 2 to edit answer");
                    println!("Type 3 to edit topics");
                    println!("Type 4 to exit");
            
                    let edit_operation: Result<usize, _> = self.input.ask_question().parse();
                    if edit_operation.clone().is_err() || edit_operation.clone().unwrap() > 4 || edit_operation.clone().unwrap() < 0  {
                        println!("Invalid input");
                        continue;
                    }
            
                    match edit_operation.unwrap() {
                        1 => {
                            println!("Current question is {}, what would you like to change it to?", card.question);
                            let new_question = self.input.ask_question();
                            card.question = new_question; 
                        }
                        2 => {
                            println!("Current answer is {}, what would you like to change it to?", card.awnser);
                            let new_awnser = self.input.ask_question();
                            card.awnser = new_awnser; 
                        }
                        3 => {
                            println!("Current topics are:");
                            for key in card.topics.clone() {
                                println!("{}", self.topics.get(key).unwrap().content);
                            }
            
                            println!("\nPress 1 to remove a topic");
                            println!("Press 2 to add a topic");
                            println!("Press 3 to exit");
                            let topic_operation: Result<usize, _> = self.input.ask_question().parse();
                            if topic_operation.clone().is_err() || topic_operation.clone().unwrap() > 3 || topic_operation.clone().unwrap() < 0  {
                                println!("Invalid input");
                                continue;
                            }
                            match topic_operation.unwrap() {
                                1 => {
                                    println!("What topic do you want to delete?");
                                    let topic_to_delete = self.input.ask_question().to_lowercase();
            
                                    
                                    card.topics.retain(|&key| {
                                        let topic = self.topics.get(key).unwrap();
                                        topic.content.to_lowercase() != topic_to_delete
                                    });
                                }
                                2 => {
                                    for (_, topic) in &self.topics {
                                        println!("{}", topic.content);
                                    }
    
                                    println!("What topics do you want it to be in? (split it from comma)");
                                    let topic_string = self.input.ask_question();
                                    let topics: Vec<&str> = topic_string.split(",").collect();
                                    
                                    let mut final_topic_keys: Vec<TopicKey> = vec![];
                                    for listed_topic in topics {
                                        for (key, topic) in &mut self.topics {
                                            if topic.content.trim().to_lowercase() == listed_topic.to_lowercase().to_string(){
                                                topic.qna.push(*card_key);
                                                final_topic_keys.push(key);
                                            }
                                        }
                                    }
                        
                                    self.cards.get_mut(*card_key).unwrap().topics.extend(final_topic_keys);
                                    
                                }
                                _ => {}
                            }
                        }
                        4 => {
                            break; 
                        }
                        _ => {}
                    }
            
                    break;
                }
            }
            
            Operations::List => {
                for (i, (_, card)) in self.cards.iter().enumerate() {
                    let mut all_topics = String::new();
                    for key in card.topics.clone() {
                        all_topics.push_str(&self.topics.get(key).unwrap().content);
                        all_topics.push_str(" ");
                    }
                    println!(
                        "\n{}: question: {}, answer: {}, topics: {}",
                        i, card.question, card.awnser, all_topics
                    );
                }
                
            },
            Operations::Topic => {
                println!("What topic do you want to add?");
                let topic_to_add = self.input.ask_question();
                self.topics.insert(Topic { content: topic_to_add, enabled: false, qna: vec![] });
            },
            Operations::Test => {
                for key in &self.quiz.cards {
                    println!("{}", self.cards.get(*key).unwrap().question);
                    if &ask_question() == &self.cards.get(*key).unwrap().awnser {
                        println!("Correct!")
                    }
                }
            },
            Operations::None => {},
            Operations::Delete => {
                for (_, value) in &self.topics {
                    println!("\ntopic: {}", value.content);
                }
                println!("\n");
    
                println!("What topic do you want to delete?");
                let topic_to_delete = self.input.ask_question().to_lowercase().to_string();
                self.topics.retain(|_, value| value.content.to_lowercase().to_string() != topic_to_delete);
            
            },
            Operations::Add => {
                for (_, topic) in &self.topics {
                    println!("{}", topic.content)
                }
                print!("\nwhat topics do you want to add to the test? (split by comma)");
                let topic_string = self.input.ask_question();
                let topics: Vec<&str> = topic_string.split(",").collect();
    
    
                // let mut final_topic_keys: Vec<TopicKey> = vec![];
                for listed_topic in topics {
                    for (key, topic) in &self.topics {
                        println!("A");
                        if topic.content.trim().to_lowercase() == listed_topic.to_lowercase().to_string(){
                            println!("B");
                            for key in &topic.qna {
                                println!("C");
                                self.quiz.cards.push(*key);
                                println!("{:?}", self.quiz.cards)
                            }
                        }
                    }
                }
    
    
            },
            Operations::Auto => {
                
            },    
        }
    }
    fn start(mut self) {
        loop {
        let mut operation =  Operations::None;
        loop {
            println!("Operations:");
            println!("(M): Make a flashcard");
            println!("(A): Autoadds cards to topic list");
            println!("(L): List flashcards");
            println!("(P): Create a topic");
            println!("(D): Delete a topic");
            println!("(E): Edit or delete a flashcard");
            println!("(Q): Add topics to a test");
            println!("(T): Start a test");
            println!("What operation would you like to do?");
            operation = match self.input.ask_question().trim().to_lowercase().as_str() {
                "t" => Operations::Test,
                "p" => Operations::Topic,
                "l" => Operations::List,
                "q" => Operations::Add,
                "e" => Operations::Edit,
                "a" => Operations::Auto,
                "d" => Operations::Delete,
                "m" => Operations::Make,
                _ => Operations::None,
            };
            if operation == Operations::None {
                break
            }
            self.operate(operation);
        }
    }
    }
 }

fn main(){
    App::new().start();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInput {
        responses: Vec<String>,
    }
    
    impl MockInput {
        fn new(responses: Vec<&str>) -> Self {
            Self {
                responses: responses.into_iter().map(|s| s.to_string()).collect(),
            }
        }
    }
    
    impl UserInput for MockInput {
        fn ask_question(&mut self) -> String {
            let response = self.responses.remove(0);
            println!("{}", response);
            response
        }
    }
    impl Default for App<MockInput> { 
        fn default() -> Self {
            Self {
                input: MockInput { responses: vec![] },
                ..Default::default()
            }
        }
    }

    #[test]
    fn edit_topic(){

        let binding = MockInput::new(["0", "3", "2", "topic"].to_vec());
        let mut app = App { input: binding, cards: SlotMap::with_key(), topics: SlotMap::with_key(), quiz: Quiz::default(), current_topics: vec![]  };
        let key = app.cards.insert(Flashcard { question: "Question".to_string(), awnser: "Awnser".to_string(), topics: vec![] });
        app.topics.insert(Topic { content: "topic".to_string(), enabled: false, qna: vec![] });
        app.operate(Operations::Edit);
        assert_eq!(app.topics.get(*app.cards.get(key).unwrap().topics.get(0).unwrap()).unwrap().content, "topic")
        //assert_eq!()
    }
    // #[test]
    // fn start_test(){
    //     let binding = MockInput::new(["topic",].to_vec());
    //     let mut app = App { input: binding, cards: SlotMap::with_key(), topics: SlotMap::with_key(), quiz: Quiz::default()  };
    //     let topic = app.topics.insert(Topic { content: "topic".to_string(), enabled: false, qna: vec![] });
    //     let card_key = app.cards.insert(Flashcard { question: "Question".to_string(), awnser: "Awnser".to_string(), topics: vec![topic] });
    //     app.operate(Operations::Add);

    // }
}