use crate::{
    server::messages::wss_message::WSSMessage, 
    storage::operations::game::{get_running_game_by_name, replace_game}, 
    game::core::{types::moves::{Move, TechMove, BugMove}, game::Player}
};


pub fn get_state(game_name: String) -> WSSMessage {
    let game = get_running_game_by_name(&game_name);
    
    if game.is_none() {
        return WSSMessage::Error("Game not fund".to_string());
    }
    
    WSSMessage::State(game.unwrap())
}


pub fn setup_base(game_name: String, x: i32, y: i32) -> WSSMessage {
    let game = get_running_game_by_name(&game_name);
    
    if game.is_none() {
        return WSSMessage::Error("Game not fund".to_string());
    }
    let mut game = game.unwrap();

    let current_state = game.states.last_mut().unwrap();
    if current_state.player_turn == Player::First {
        if x > (current_state.map.len() as i32 / 2) {
            return WSSMessage::Error("Place base on own side".to_string());
        }
        current_state.move_que.push(Move::Tech(TechMove::SetupMove(x, y)));
    } else {
        if x < (current_state.map.len() as i32 / 2) {
            return WSSMessage::Error("Place base on own side".to_string());
        }
        current_state.move_que.push(Move::Bug(BugMove::SetupMove(x, y)));
    }

    replace_game(game_name, game.clone());
    WSSMessage::State(game)
}

pub fn next_phase(game_name: String) -> WSSMessage {
    let game = get_running_game_by_name(&game_name);
    
    if game.is_none() {
        return WSSMessage::Error("Game not fund".to_string());
    }
    let mut game = game.unwrap();
    if let Err(e) = game.progress_state() {
        return WSSMessage::Error(format!("{:#?}", e));
    }
    replace_game(game_name, game.clone());
    WSSMessage::State(game)
}

pub fn undo_move(game_name: String) -> WSSMessage {
    let game = get_running_game_by_name(&game_name);
    
    if game.is_none() {
        return WSSMessage::Error("Game not fund".to_string());
    }
    let mut game = game.unwrap();
    let current_state = game.states.last_mut().unwrap();
    if !current_state.move_que.is_empty() {
        current_state.move_que.pop();
    }
    replace_game(game_name, game.clone());
    WSSMessage::State(game)
}