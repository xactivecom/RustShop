use std::sync::Arc;

// Serialize JSON data
use serde::{ Serialize, Deserialize };

pub mod utils;

// Message types that can be sent
// The PartialEq trait is used for comparison ops
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Client {
    Join{
        chat_name: Arc<String>,
    },
    Post{
        chat_name: Arc<String>,
        message: Arc<String>,
    }
}

// Message types that can be received
// The PartialEq trait is used for comparison ops
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}