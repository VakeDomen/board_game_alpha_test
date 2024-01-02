use crate::{
    storage::operations::socket::authenticate_socket, 
    server::messages::{wss_message::WSSMessage, control_commands::ControlCommand, game_commands::GameCommand}
};

use super::{control_message::{create_game, join_game, start_game}, game_message::get_state};


pub fn handle(msg: WSSMessage, socket_id: String) -> WSSMessage {
    match msg {
        WSSMessage::Game(name, g) => handle_game_message(name, g),
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

fn handle_game_message(game_name: String, msg: GameCommand) -> WSSMessage {
    match msg {
        GameCommand::GetState => get_state(game_name, msg),
        GameCommand::InvalidCommand(e) => WSSMessage::Error(e),
        _ => WSSMessage::Success(false),
    }
}
