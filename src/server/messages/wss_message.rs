use std::collections::HashMap;

use serde::Serialize;
use serde_any::Format;
use tokio_tungstenite::tungstenite::Message;


use crate::game::{core::{lobby::new_game::NewGame, game::Game}, game_models::types::structure::{StructureRecepie, StructureSelector}};

use super::{control_commands::ControlCommand, game_commands::GameCommand};

#[derive(Debug, Serialize)]
pub enum WSSMessage {
    // from client
    Game(String, GameCommand),
    Control(ControlCommand),
    Unknown,

    // to client
    Success(bool),
    Error(String),
    NewGame(NewGame),
    State(Game),
    StructureRecepeies(HashMap<StructureSelector, StructureRecepie>),
    Lobby(Vec<NewGame>),
    Running(Vec<Game>),
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

        // parse game command
        if message_string.starts_with("GAME ") {
            let tokens: Vec<&str> = message_string.splitn(3, ' ').collect();
            if tokens.len() < 3 {
                println!("Invalid GAME command format.");
                return Self::Unknown;
            }

            let game_name = tokens[1].to_string();
            let command_str = tokens[2].to_string();
            let command = GameCommand::from(command_str);

            return Self::Game(game_name, command);
        } 

        // parse control command
        if message_string.starts_with("CONTROL ") {
            return Self::Control(ControlCommand::from(message_string["CONTROL ".len()..].to_string()));
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