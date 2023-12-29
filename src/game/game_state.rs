use super::{game::Game, new_game::NewGame};

pub enum GameState {
    Lobby(NewGame),
    Running(Game)
}