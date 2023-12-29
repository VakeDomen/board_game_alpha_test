use super::{
    message::WSSMessage, 
    control_message::ControlMessage, 
    control_message_handler::{
        authenticate_socket, 
        create_game, 
        join_game, 
        start_game
    }
};

pub fn handle(msg: WSSMessage, socket_id: String) -> WSSMessage {
    match msg {
        WSSMessage::Game(_) => handle_game_message(msg),
        WSSMessage::Control(c) => handle_control_message(c, socket_id),
        _ => return WSSMessage::Unknown,
    }
}

fn handle_control_message(msg: ControlMessage, socket_id: String) -> WSSMessage {
    match msg {
        ControlMessage::Authenticate(name) => authenticate_socket(name, socket_id),
        ControlMessage::CreateGame(name) => create_game(name, socket_id),
        ControlMessage::JoinGame(name) => join_game(name, socket_id),
        ControlMessage::StartGame(name) => start_game(name, socket_id),
        ControlMessage::Unknown => WSSMessage::Unknown,
    }
}

fn handle_game_message(msg: WSSMessage) -> WSSMessage {
    WSSMessage::Success(false)
}