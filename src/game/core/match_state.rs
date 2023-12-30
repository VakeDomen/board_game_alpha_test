use super::{game::Game, new_game::NewGame};

pub enum MatchState {
    Lobby(NewGame),
    Running(Game)
}