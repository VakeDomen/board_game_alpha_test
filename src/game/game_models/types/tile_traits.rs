use crate::game::{core::game::GameState, game_models::functions::ability_active::{contains_required_resources, remove_resources}};

use super::{structure::Structure, resource::Resouce};

pub trait Placable {
    fn place(self, game_state: &mut GameState, x: i32, y: i32) -> Option<Structure>;
    fn can_place_on(self, game_state: &GameState, x: i32, y: i32) -> bool;
    fn has_enough_resources(self, game_state: &GameState) -> bool;
}

pub trait Upgradable {
    fn upgrade(self, game_state: &mut GameState, structure: &mut Structure) -> bool;
    fn can_upgrade(self, game_state: &GameState, structure: &mut Structure) -> bool;
}

pub trait ActiveAbility {
    fn trigger(self, game_state: &mut GameState, structure: &mut Structure) -> bool;
    
    fn activate(&self, game_state: &mut GameState, structure: &mut Structure, payment: Vec<Resouce>) -> bool {
        if !self.can_activate(game_state, structure, &payment) {
            return false;
        }

        if !remove_resources(&mut game_state.tech_resources, &payment) {
            return false;
        }

        structure.activated = true;
        structure.activation_resources.push(Resouce::Gold);
        
        true
    }
    fn can_activate(&self, game_state: &GameState, structure: &Structure, payment: &Vec<Resouce>) -> bool {
        if structure.activated {
            return false
        }

        if !contains_required_resources(&game_state.tech_resources, &payment) {
            return false;
        }
        true
    }
    
    fn can_trigger(&self, game_state: &GameState, structure: &Structure, payment: &Vec<Resouce>) -> bool{
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
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool;
}