use crate::game::core::game::GameState;

use super::structure::Structure;

pub trait Placable {
    fn place(self, game_state: GameState) -> Option<Structure>;
    fn can_place_on(self, game_state: GameState, x: i32, y: i32) -> bool;
    fn has_enough_resources(self, game_state: GameState) -> bool;
}

pub trait Upgradable {
    fn upgrade(self, game_state: GameState, structure: &mut Structure) -> Option<Structure>;
    fn can_upgrade(self, game_state: GameState, structure: &mut Structure, x: i32, y: i32) -> bool;
    fn has_enough_resources_for_upgrade(self, game_state: GameState, structure: &mut Structure) -> bool;
}