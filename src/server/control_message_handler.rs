use crate::{game::{new_game::NewGame, game_state::GameState}, storage::{active::GAMES, operations_socket::{get_socket_name, is_authenticated}}};

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

    let game = NewGame {
        name,
        player1: get_socket_name(&socket_id).unwrap(),
        player2: None,
    };

    let mut games = GAMES.lock().unwrap();
    games.push(GameState::Lobby(game.clone()));
    WSSMessage::NewGame(game)
}

