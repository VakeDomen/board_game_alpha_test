use crate::game::core::game::GameState;

use super::structure::Structure;

pub trait Placable {
    fn place(self, game_state: GameState, x: i32, y: i32) -> Option<Structure>;
    fn can_place_on(self, game_state: GameState, x: i32, y: i32) -> bool;
    fn has_enough_resources(self, game_state: GameState) -> bool;
}

pub trait Upgradable {
    fn upgrade(self, game_state: GameState, structure: &mut Structure) -> bool;
    fn can_upgrade(self, game_state: GameState, structure: &mut Structure) -> bool;
}

pub trait Acivatable {
    fn activate(self, game_state: GameState, structure: &mut Structure) -> bool;
    fn can_activate(self, game_state: GameState, structure: &mut Structure) -> bool;
    fn trigger(self, game_state: GameState, structure: &mut Structure) -> bool;
    fn can_trigger(self, game_state: GameState, structure: &mut Structure) -> bool;
}