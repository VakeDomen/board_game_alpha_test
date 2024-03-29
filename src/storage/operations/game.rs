use crate::{
    storage::active::{MATCHES, save_matches}, 
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

pub fn get_other_player(game_name: String, player_name: String) -> Option<String> {
    let mut games = MATCHES.lock().unwrap();
    for game in games.iter_mut() {
        if let MatchState::Lobby(g) = game {
            if g.name == game_name {
                if g.player1 == player_name {
                    return g.player2.clone();
                }
                if let Some(p2) = g.player2.clone() {
                    if p2 == player_name {
                        return Some(g.player1.clone());
                    }
                }
            }
        }
    }
    None
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

        if let MatchState::Running(g) = game {
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

pub fn game_exists(name: String) -> bool {
    let games = MATCHES.lock().unwrap();
    for game in games.iter() {
        if let MatchState::Lobby(g) = game {
            if g.name == name {
                return true;
            }
        }

        if let MatchState::Running(g) = game {
            if g.name == name {
                return true;
            }
        }
    }
    false
}

pub fn add_to_storage(game: MatchState) {
    let mut games = MATCHES.lock().unwrap();
    games.push(game);
}

pub fn get_running_game_by_name(game_name: &str) -> Option<Game> {
    let matches = MATCHES.lock().unwrap();
    for game in matches.iter() {
        if let MatchState::Running(g) = game {
            if g.name == game_name {
                return Some(g.clone());
            }
        }
    }
    None
}

pub fn replace_game(name: String, game: Game) {
    remove_from_storage(name);
    add_to_storage(MatchState::Running(game));
    save_matches();
}