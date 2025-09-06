use std::collections::HashMap;

use async_std::sync::{ Arc, Mutex };

use crate::chats::Chats;


pub struct ChatTracker(Mutex<HashMap<Arc<String>, Arc<Chats>>>);

impl ChatTracker {
    pub fn new() -> ChatTracker {
        ChatTracker(Mutex::new(HashMap::new()))
    }

    // Find the chat
    pub fn find(&self, name: &String) -> Option<Arc<Chats>> {
        self.0.lock().unwrap().get(name).cloned()
    }

    // Find the chat and return, or return a new chat instance
    pub fn find_or_new(&self, name: Arc<String>) -> Arc<Chats> {
        self.0.lock().unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Chats::new(name)))
            .clone()
    }
}
