use crate::game::core::game::Game;
use super::new_game::NewGame;


pub enum MatchState {
    Lobby(NewGame),
    Running(Game)
}