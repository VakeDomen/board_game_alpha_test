use crate::{
    game::core::{lobby::{new_game::NewGame, match_state::MatchState}, game::Game}, 
    storage::{operations::{
        socket::{is_authenticated, get_socket_name}, 
        game::{has_second_player, get_lobby_game_by_name, remove_from_storage, add_to_storage, set_player2}
    }, active::save_matches}
};

use crate::server::messages::wss_message::WSSMessage;

pub fn start_game(name: String, socket_id: String) -> WSSMessage {
    if !is_authenticated(&socket_id) {
        return WSSMessage::Unauthorized;
    }

    if !has_second_player(&name) {
        return WSSMessage::NotEnoughPlayers;
    }

    let game = get_lobby_game_by_name(name.clone());
    if game.is_none() {
        return WSSMessage::Success(false);
    }
    let game = game.unwrap();
    let game = Game::from(game);

    remove_from_storage(name);
    add_to_storage(MatchState::Running(game));
    save_matches();
    WSSMessage::Success(true)
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
    save_matches();
    WSSMessage::NewGame(get_lobby_game_by_name(name).unwrap())
}

pub fn create_game(name: String, socket_id: String) -> WSSMessage {
    if !is_authenticated(&socket_id) {
        return WSSMessage::Unauthorized;
    }

    let get_socket_name = get_socket_name(&socket_id);
    let game = NewGame {
        name,
        player1: get_socket_name.unwrap(),
        player2: None,
    };

    add_to_storage(MatchState::Lobby(game.clone()));
    save_matches();
    WSSMessage::NewGame(game)
}

