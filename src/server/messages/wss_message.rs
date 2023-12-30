use serde::Serialize;
use serde_any::Format;
use tokio_tungstenite::tungstenite::Message;


use crate::game::core::lobby::new_game::NewGame;

use super::control_message::ControlMessage;

#[derive(Debug, Serialize)]
pub enum WSSMessage {
    // from client
    Game(String),
    Control(ControlMessage),
    Unknown,

    // to client
    Success(bool),
    NewGame(NewGame),
    Unauthorized,
    NotEnoughPlayers,
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

        match message_string.as_str() {
            _ if message_string.starts_with("GAME ") => Self::Game(message_string["GAME ".len()..].to_string()),
            _ if message_string.starts_with("CONTROL ") => Self::Control(ControlMessage::from(message_string["CONTROL ".len()..].to_string())),
            _ => Self::Unknown,
        }
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