use crate::storage::active::SOCKETS;
use super::{message::WSSMessage, control_message::ControlMessage};

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
        ControlMessage::CreateGame(name) => todo!(),
        ControlMessage::JoinGame(name) => todo!(),
        ControlMessage::StartGame(name) => todo!(),
        ControlMessage::Unknown => WSSMessage::Unknown,
    }
}

fn authenticate_socket(new_name: String, socket_id: String) -> WSSMessage {
    let mut socket_data = SOCKETS.lock().unwrap();
    for (id, data) in socket_data.iter_mut() {
        if *id == socket_id {
            data.1 = Some(new_name.clone()); // Dereference to modify
            return WSSMessage::Success(true)
        }
    }
    WSSMessage::Success(false) // Assuming this is a valid return value
}


fn handle_game_message(msg: WSSMessage) -> WSSMessage {
    WSSMessage::Success(false)
}