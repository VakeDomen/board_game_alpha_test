use crate::game::{game_models::functions::ability_active::{contains_required_resources, remove_resources}, core::game_state::GameState};

use super::{tile::{Tile, NewTile}, resource::Resource, map::MapError};



pub trait Placable {
    fn place(&self, new_tile: NewTile, game_state: &mut GameState, x: i32, y: i32) -> Result<Tile, MapError>;
    fn can_place_on(&self, new_tile: &NewTile, game_state: &GameState, x: i32, y: i32) -> Result<(), MapError>;
}

pub trait Upgradable {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Tile) -> bool;
    fn can_upgrade(&self, game_state: &GameState, structure: &mut Tile) -> bool;
}

pub trait ActiveAbility {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Tile) -> bool;
    fn activate(&self, game_state: &mut GameState, structure: &mut Tile, payment: Vec<Resource>) -> bool {
        if !self.can_activate(game_state, structure, &payment) {
            return false;
        }

        if !remove_resources(&mut game_state.tech_resources, &payment) {
            return false;
        }

        structure.activated = true;
        structure.activation_resources = payment;
        
        true
    }


    fn can_activate(&self, game_state: &GameState, structure: &Tile, payment: &Vec<Resource>) -> bool {
        if structure.activated {
            return false
        }

        if !contains_required_resources(&game_state.tech_resources, &payment) {
            return false;
        }
        true
    }
    
    fn can_trigger(&self, _: &GameState, structure: &Tile, payment: &Vec<Resource>) -> bool{
        if structure.activated {
            return false
        }

        if !contains_required_resources(&structure.activation_resources, &payment) {
            return false;
        }
        true
    }
}

pub trait PassiveAbility {
    fn activate_passive(&self, game_state: &mut GameState, structure: &mut Tile) -> bool;
}