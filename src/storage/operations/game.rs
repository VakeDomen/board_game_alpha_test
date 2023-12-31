use crate::{
    storage::active::MATCHES, 
    game::core::{lobby::{new_game::NewGame, match_state::MatchState}, 
    game::Game}
};



pub fn get_new_games() -> Vec<NewGame> {
    let mut new_games = vec![];
    let games = MATCHES.lock().unwrap();
    for game in games.iter() {
        if let MatchState::Lobby(ng) = game {
            new_games.push(ng.clone());
        }
    }
    new_games
}


pub fn get_running_games(player_name: String) -> Vec<Game> {
    let mut running_games = vec![];
    let games = MATCHES.lock().unwrap();
    for game in games.iter() {
        if let MatchState::Running(g) = game {
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
    let mut games = MATCHES.lock().unwrap();
    for game in games.iter_mut() {
        if let MatchState::Lobby(g) = game {
            if g.name == game_name {
                g.player2 = Some(player_name.clone());
            }
        }
    }
}   

pub fn has_second_player(name: &str) -> bool {
    let game = get_lobby_game_by_name(name.to_string());
    if !game.is_some() {
        return false;
    }
    game.unwrap().player2.is_some()
}

pub fn remove_from_storage(name: String) {
    let mut games = MATCHES.lock().unwrap();
    let mut index_to_delete = None;
    for (index, game) in games.iter().enumerate() {
        if let MatchState::Lobby(g) = game {
            if g.name == name {
                index_to_delete = Some(index);
                break;
            }
        }
    }
    if index_to_delete.is_some() {
        games.remove(index_to_delete.unwrap());
    }
}

pub fn add_to_storage(game: MatchState) {
    let mut games = MATCHES.lock().unwrap();
    games.push(game);
}