use std::collections::HashMap;

use crate::game::{game_models::{types::{structure::{StructureSelector, Structure}, tile_traits::ActiveAbility, resource::Resource, map::Interactor}, data::structures::recepies::get_recepie}, core::game_state::GameState};

pub struct TechRefinery1Active;
pub struct TechRefinery2Active;
pub struct TechMarketActive;
pub struct TechNukeActive;
pub struct BugBase3Active;


pub fn get_active_abilities() -> HashMap<StructureSelector, Option<Box<dyn ActiveAbility>>> {
    let mut hm: HashMap<StructureSelector, Option<Box<dyn ActiveAbility>>> = HashMap::new();
    hm.insert(StructureSelector::BugBase1, None);
    hm.insert(StructureSelector::BugBase2, None);
    hm.insert(StructureSelector::BugBase3, None);
    hm.insert(StructureSelector::TechBase, None);
    hm.insert(StructureSelector::TechRoad, None);
    hm.insert(StructureSelector::TechMine1, None);
    hm.insert(StructureSelector::TechMine2, None);
    hm.insert(StructureSelector::TechRefinery1, Some(Box::new(TechRefinery1Active{})));
    hm.insert(StructureSelector::TechRefinery2, Some(Box::new(TechRefinery2Active{})));
    hm.insert(StructureSelector::TechMarket, Some(Box::new(TechMarketActive{})));
    hm.insert(StructureSelector::TechTurret1, None);
    hm.insert(StructureSelector::TechTurret2, None);
    hm.insert(StructureSelector::TechArtillery1, None);
    hm.insert(StructureSelector::TechArtillery2, None);
    hm.insert(StructureSelector::TechWall1, None);
    hm.insert(StructureSelector::TechNuke, Some(Box::new(TechNukeActive{})));
    hm
}

impl ActiveAbility for BugBase3Active {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !self.can_trigger(game_state, structure, &vec![
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
        ]) {
            return false;
        }

        game_state.bug_resources.push(Resource::GiantEgg);

        structure.structure_type = StructureSelector::BugBase2;
        game_state.map.remove_tile(structure.id.clone());
        let recepie = get_recepie(&structure.structure_type);
        for (location, _) in game_state.map.get_footprint_tiles(structure.x, structure.y, &recepie.footprint) {
            game_state.map[location.0][location.1] = structure.id.to_string();
        }

        true
    }
}

impl ActiveAbility for TechNukeActive {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !self.can_trigger(game_state, structure, &vec![Resource::Metal, Resource::Metal, Resource::Metal]) {
            return false;
        }

        structure.activated = false;
        structure.activation_resources = vec![];

        let x = structure.additional_data.get("nuke_target_x");
        let y = structure.additional_data.get("nuke_target_y");

        let x: i32 = match x {
            Some(x) => x.parse().unwrap(),
            None => return false,
        };

        let y: i32 = match y {
            Some(y) => y.parse().unwrap(),
            None => return false,
        };
        todo!("Shoot nuke");
        true
    }
}

impl ActiveAbility for TechMarketActive {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        let mut trigger_mode = 0;
        if self.can_trigger(game_state, structure, &vec![Resource::Gold]) {
            trigger_mode = 1;
        }
        if self.can_trigger(game_state, structure, &vec![Resource::Metal]) {
            trigger_mode = 2;
        }
        if trigger_mode == 0 {
            return false;
        }

        structure.activated = false;
        structure.activation_resources = vec![];
            
        // deconstruct building
        if trigger_mode == 1 {
            let deconstruct_structure_id = structure.additional_data.remove("deconstruct_id");
            let deconstruct_structure_id = match deconstruct_structure_id {
                Some(id) => id,
                None => return false,
            };

            game_state.tiles.remove(&deconstruct_structure_id);
            return game_state.map.remove_tile(deconstruct_structure_id);
        }

        // sell metal
        if trigger_mode == 2 {
            game_state.tech_resources.push(Resource::Gold);
            game_state.tech_resources.push(Resource::Gold);
            game_state.tech_resources.push(Resource::Gold);            
        }
        
        true
    }
}


impl ActiveAbility for TechRefinery2Active {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !self.can_trigger(game_state, structure, &vec![Resource::Gold]) {
            return false;
        }

        structure.activated = false;
        structure.activation_resources = vec![];
        game_state.tech_resources.push(Resource::Metal);
        game_state.tech_resources.push(Resource::Metal);

        true
    }
}

impl ActiveAbility for TechRefinery1Active {
    fn trigger(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !self.can_trigger(game_state, structure, &vec![Resource::Gold]) {
            return false;
        }
        structure.activated = false;
        structure.activation_resources = vec![];
        game_state.tech_resources.push(Resource::Metal);
        true
    }
}

pub fn contains_required_resources(game_resources: &Vec<Resource>, required_resources: &Vec<Resource>) -> bool {
    let game_counts = to_counts(game_resources);
    let required_counts = to_counts(required_resources);

    // Check if the game resources meet or exceed the required counts
    required_counts.into_iter().all(|(res, count)| game_counts.get(&res).unwrap_or(&0) >= &count)
}

// Function to convert a Vec<Resource> into a HashMap<Resource, i32> to count occurrences
fn to_counts(resources: &Vec<Resource>) -> HashMap<Resource, i32> {
    let mut counts = HashMap::new();
    for resource in resources {
        *counts.entry(resource.clone()).or_insert(0) += 1;
    }
    counts
}


pub fn remove_resources(game_resources: &mut Vec<Resource>, required_resources: &Vec<Resource>) -> bool {
    let mut required_counts = to_counts(required_resources);

    // Check if we have enough resources to remove
    for resource in required_resources {
        let count = required_counts.entry(resource.clone()).or_default();
        if *count > 0 {
            *count -= 1;
        } else {
            // Not enough resources; bail out early
            return false;
        }
    }

    // If we have enough of each resource, proceed to remove them
    for (resource, count) in required_counts {
        for _ in 0..count {
            if let Some(pos) = game_resources.iter().position(|x| *x == resource) {
                game_resources.remove(pos);
            }
        }
    }

    true
}
