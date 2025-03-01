use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::keyboard::KeyboardReader;
use crate::llm::SimpleModel;
use crate::println;

lazy_static! {
    static ref MODEL: Mutex<SimpleModel> = Mutex::new(SimpleModel::new());
    static ref CHAT_HISTORY: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
}

pub struct ChatInterface {
    keyboard: KeyboardReader,
}

impl ChatInterface {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardReader::new(),
        }
    }
    
    pub fn start_chat(&mut self) {
        // Display welcome message
        println!("\n==================================");
        println!("Welcome to Bare Metal LLM Chat OS");
        println!("==================================\n");
        
        loop {
            println!("\nYou: ");
            let input = self.keyboard.read_line();
            
            // Exit command
            if input.trim() == "/exit" {
                println!("\nShutting down chat interface...");
                break;
            }
            
            // Process input and generate response
            let user_input = String::from(input);
            let model = MODEL.lock();
            let response = model.process_input(&user_input);
            drop(model); // Explicitly drop the lock
            
            // Store in chat history
            CHAT_HISTORY.lock().push((user_input, response.clone()));
            
            // Display response
            println!("\nLLM: {}", response);
            
            // Display token usage for demonstration
            println!("\n[Token count: {}]", MODEL.lock().get_token_count());
        }
    }
    
    pub fn show_history(&self) {
        let history = CHAT_HISTORY.lock();
        
        if history.is_empty() {
            println!("No chat history available.");
            return;
        }
        
        println!("\n--- Chat History ---");
        for (i, (user, llm)) in history.iter().enumerate() {
            println!("Message {}:", i + 1);
            println!("You: {}", user);
            println!("LLM: {}", llm);
            println!("-------------------");
        }
    }
}