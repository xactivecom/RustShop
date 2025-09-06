
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{ Arc, Mutex };

use chat::utils::{ self, ChatResult };
use chat::{ Client, Server };
use crate::chats_map::ChatTrcker;


// This struct represents an outbound TCP stream
pub struct Leaving(Mutex<TcpStream>);

// Implementation for leaving
impl Leaving {
    pub fn new(client: TcpStream) -> Leaving {
        // Wrap TCP stream with a mutex to enforce single-thread usage
        Leaving(Mutex::new(client))
    }

    // Send a packet over TCP and return result
    pub async fn send(&self, packet: Server) -> ChatResult<()> {
        // Get protected TCP stream so that only one task can use it at a time
        let mut lock = self.0.lock().await;

        // Send JSON data to server and wait for result
        utils::send_json(&mut *lock, &packet).await?;

        // Flush buffer data
        lock.flush().await?;

        Ok(())
    }
}

// Handle connections
pub async fn handle(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    // Create atomic ref counter for cloned socket
    let leaving = Arc::new(Leaving::new(socket.clone()));

    // Create buffer reader
    let buffered = BufReader::new(socket);

    // Create request receiver
    let mut from_client = utils::receive(buffered);

    // Handle requests from clients
    while let Some(req_res) = from_client.next().awit {
        let request = req_res?;

        let result = match request {
            Client::Join { chat_name } => {
                // Handle join chat request
                let chat = chats.find_or_new(chat_name);
                chat.join(leaving.clone());
                Ok(())
            },
            Client::Post { chat_name, message } => match chats.find(&chat_name) {
                // Handle post chat request
                Some(chat) => {
                    chat.post(message);
                    Ok(())
                },
                None => Err(format!("Chat does not exist: {}", chat_name)),
            },
        };

        // If error reply to client
        if let Err(message) = result {
            let report = Server::Error(message);
            leaving.send(report).await?;
        }
    }
    Ok(())
}
