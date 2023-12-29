use crate::storage::active::SOCKETS;

use super::message::WSSMessage;

pub fn start_game(name: String, socket_id: String) -> WSSMessage {
    if !is_authenticated(&socket_id) {
        return WSSMessage::Unauthorized;
    }
    WSSMessage::Unknown
}

pub fn join_game(name: String, socket_id: String) -> WSSMessage {
    if !is_authenticated(&socket_id) {
        return WSSMessage::Unauthorized;
    }
    WSSMessage::Unknown
}

pub fn create_game(name: String, socket_id: String) -> WSSMessage {
    if !is_authenticated(&socket_id) {
        return WSSMessage::Unauthorized;
    }
    WSSMessage::Unknown
}

pub fn authenticate_socket(new_name: String, socket_id: String) -> WSSMessage {
    let mut socket_data = SOCKETS.lock().unwrap();
    for (id, data) in socket_data.iter_mut() {
        if *id == socket_id {
            data.1 = Some(new_name.clone()); // Dereference to modify
            return WSSMessage::Success(true)
        }
    }
    WSSMessage::Success(false) // Assuming this is a valid return value
}

pub fn is_authenticated(socket_id: &String) -> bool {
    let socket_data = SOCKETS.lock().unwrap();
    let s = socket_data.get(socket_id);
    if let Some(s) = s {
        return s.1.is_some();
    }
    false
}