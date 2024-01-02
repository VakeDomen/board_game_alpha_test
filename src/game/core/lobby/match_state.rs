use serde::{Serialize, Deserialize};

use crate::game::core::game::Game;
use super::new_game::NewGame;


#[derive(Debug, Serialize, Deserialize)]
pub enum MatchState {
    Lobby(NewGame),
    Running(Game)
}