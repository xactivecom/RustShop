use std::error::Error;
use std::marker::Unpin;

// Asnyc version of std libraries
use async_std::prelude::*;

// Serialize JSON data
use serde::de::DeserializeOwned;
use serde::Serialize;

// Error type consisting of a Box smart pointer with:
// - dyn trait for heap allocation and reference counting
// - Error trait for caught std Rust errors
// - Send/Sync marker traits for error to share between threads
// - static means error must last as long as the entire program lifetime
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

// Result type consisting of a generic type or chat error:
pub type ChatResult<T> = Result<T, ChatError>;

// Send a JSON representation as a string
// The outbound stream must be writable and unpinable
// The packet must be serializable
pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    // Serialize packet into JSON string and append newline
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    // Write data to byte stream
    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}

// Receive a generic data type
// The inbound stream must implement buffered read and unpinable
// The data type must be deserializable and owned by the receiver
pub async fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
    I: async_std::io::BufRead + Unpin,
    T: DeserializeOwned,
{
    // Read incoming lines of incoming stream
    incoming.lines().map(|line| -> ChatResult<T> {
        let li = line?;

        // Deserialize JSON string to generic data type
        let msg = serde_json::from_str::<T>(&li)?;
        Ok(msg)
    })
}
