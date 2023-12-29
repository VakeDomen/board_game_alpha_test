use std::fmt::format;

use serde::Serialize;
use serde_any::Format;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Serialize)]
pub enum WSSMessage {
    Game(String),
    Control(String),
    Unknown,
}

impl From<Message> for WSSMessage {
    fn from(value: Message) -> Self {
        let message_string: String = match value.into_text() {
            Ok(t) => t,
            Err(e) => {
                println!("Error parsing WSS message: {:#?}", e);
                return Self::Unknown;
            },
        };

        let prefix = "GAME ";
        if message_string.starts_with(prefix) {
            let no_prefix = message_string[prefix.len()..].to_string();
            return Self::Game(no_prefix);
        }

        let prefix = "CONTROL ";
        if message_string.starts_with(prefix) {
            let no_prefix = message_string[prefix.len()..].to_string();
            return Self::Control(no_prefix);
        }

        Self::Unknown
    }
}

impl Into<Message> for WSSMessage {
    fn into(self) -> Message {
        match serde_any::to_string(&self, Format::Json)  {
            Ok(s) => Message::Text(s),
            Err(e) => Message::Text(format!("Couldn't serialize message: {:#?}", e)),
        }
    }
}