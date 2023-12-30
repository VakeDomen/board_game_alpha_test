use crate::{
    game::core::{new_game::NewGame, match_state::MatchState}, 
    storage::{active::MATCHES, 
    operations::socket::{get_socket_name, is_authenticated}, 
    operations::game::{get_lobby_game_by_name, set_player2}}};

use crate::server::messages::wss_message::WSSMessage;

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

    let game = get_lobby_game_by_name(name.clone());
    let player = get_socket_name(&socket_id);
    
    if game.is_none() {
        return WSSMessage::Success(false)
    }

    set_player2(
        game.unwrap().name,
        player.unwrap()
    );
    WSSMessage::NewGame(get_lobby_game_by_name(name).unwrap())
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

    let mut games = MATCHES.lock().unwrap();
    games.push(MatchState::Lobby(game.clone()));
    WSSMessage::NewGame(game)
}

