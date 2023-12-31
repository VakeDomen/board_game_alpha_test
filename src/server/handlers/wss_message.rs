use crate::{
    storage::operations::socket::authenticate_socket, 
    server::messages::{wss_message::WSSMessage, control_commands::ControlCommand}
};

use super::control_message::{create_game, join_game, start_game};


pub fn handle(msg: WSSMessage, socket_id: String) -> WSSMessage {
    match msg {
        WSSMessage::Game(_) => handle_game_message(msg),
        WSSMessage::Control(c) => handle_control_message(c, socket_id),
        _ => return WSSMessage::Unknown,
    }
}

fn handle_control_message(msg: ControlCommand, socket_id: String) -> WSSMessage {
    match msg {
        ControlCommand::Authenticate(name) => authenticate_socket(name, socket_id),
        ControlCommand::CreateGame(name) => create_game(name, socket_id),
        ControlCommand::JoinGame(name) => join_game(name, socket_id),
        ControlCommand::StartGame(name) => start_game(name, socket_id),
        ControlCommand::Unknown => WSSMessage::Unknown,
    }
}

fn handle_game_message(msg: WSSMessage) -> WSSMessage {
    WSSMessage::Success(false)
}
