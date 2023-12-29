use crate::game::{game_state::GameState, new_game::NewGame, game::Game};

use super::active::GAMES;

pub fn get_new_games() -> Vec<NewGame> {
    let mut new_games = vec![];
    let games = GAMES.lock().unwrap();
    for game in games.iter() {
        if let GameState::Lobby(ng) = game {
            new_games.push(ng.clone());
        }
    }
    new_games
}


pub fn get_running_games(player_name: String) -> Vec<Game> {
    let mut running_games = vec![];
    let games = GAMES.lock().unwrap();
    for game in games.iter() {
        if let GameState::Running(g) = game {
            if g.player1 == player_name || g.player2 == player_name {
                running_games.push(g.clone());
            }
        }
    }
    running_games
}

pub fn get_lobby_game_by_name(name: String) -> Option<NewGame> {
    let games = get_new_games();
    games.iter().filter(|g| g.name == name).nth(0).cloned()
}

pub fn set_player2(game_name: String, player_name: String) {

}