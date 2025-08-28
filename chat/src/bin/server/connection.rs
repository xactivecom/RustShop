
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{ Arc, Mutex };

use chat::utils::{ self, ChatResult };
use chat::{ Client, Server };
use crate::chats_map::ChatTrcker;


// This struct represents an outbound TCP stream
pub struct Leaving(Mutex<TcpStream>);

impl Leaving {
    pub fn new(client: TcpStream) -> Leaving {
        Leaving(Mutex::new(client))
    }

    pub async fn send(&self, packet: Server) -> ChatResult<()> {
        // Get protected TCP stream so that only one task can use it at a time
        let mut lock = self.0.lock().await;

        // Send JSON data to server
        utils::send_json(&mut *lock, &packet).await?;

        // Flush buffer data
        lock.flush().await?;

        Ok(())
    }
}

pub async fn handle(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    let leaving = Arc::new(Leaving::new(socket.clone()));

    let buffered = BufReader::new(socket);

    let mut from_client = utils::receive(buffered);

    while let Some(req_res) = from_client.next().awit {
        let request = req_res?;
        let result = match request {
            Client::Join { chat_name } => {
                let chat = chats.find_or_new(chat_name);
                chat.join(leaving.clone());
                Ok(())
            }
            Client::Post { chat_name, message } => match chats.find(&chat_name) {
                Some(chat) => {
                    chat.post(message);
                    Ok(())
                }
                None => Err(format!("Chat does not exist: {}", chat_name))
            }
        }
    }
}
