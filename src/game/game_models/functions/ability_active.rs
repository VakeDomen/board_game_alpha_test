use std::collections::HashMap;

use crate::game::{game_models::{types::{tile::{TileSelector, Tile}, tile_traits::ActiveAbility, resource::Resource, map::{Interactor, TileOption}}, data::recepies::get_recepie}, core::game_state::GameState};

pub struct TechRefinery1Active;
pub struct TechRefinery2Active;
pub struct TechMarketActive;
pub struct TechNukeActive;
pub struct BugBase3Active;


pub fn get_active_abilities() -> HashMap<TileSelector, Option<Box<dyn ActiveAbility>>> {
    let mut hm: HashMap<TileSelector, Option<Box<dyn ActiveAbility>>> = HashMap::new();
    hm.insert(TileSelector::BugBase1, None);
    hm.insert(TileSelector::BugBase2, None);
    hm.insert(TileSelector::BugBase3, None);
    hm.insert(TileSelector::TechBase, None);
    hm.insert(TileSelector::TechRoad, None);
    hm.insert(TileSelector::TechMine1, None);
    hm.insert(TileSelector::TechMine2, None);
    hm.insert(TileSelector::TechRefinery1, Some(Box::new(TechRefinery1Active{})));
    hm.insert(TileSelector::TechRefinery2, Some(Box::new(TechRefinery2Active{})));
    hm.insert(TileSelector::TechMarket, Some(Box::new(TechMarketActive{})));
    hm.insert(TileSelector::TechTurret1, None);
    hm.insert(TileSelector::TechTurret2, None);
    hm.insert(TileSelector::TechArtillery1, None);
    hm.insert(TileSelector::TechArtillery2, None);
    hm.insert(TileSelector::TechWall1, None);
    hm.insert(TileSelector::TechNuke, Some(Box::new(TechNukeActive{})));
    hm
}

impl ActiveAbility for BugBase3Active {
    fn trigger(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !self.can_trigger(game_state, tile, &vec![
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
        ]) {
            return false;
        }

        game_state.bug_resources.push(Resource::GiantEgg);

        tile.tile_type = TileSelector::BugBase2;
        game_state.map.remove_tile(tile.id.clone());
        let recepie = get_recepie(&tile.tile_type);
        for (location, _) in game_state.map.get_footprint_tiles(tile.x, tile.y, &recepie.footprint) {
            game_state.map[location.0][location.1] = tile.id.to_string();
        }

        true
    }
}

impl ActiveAbility for TechNukeActive {
    fn trigger(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !self.can_trigger(game_state, tile, &vec![Resource::Metal, Resource::Metal, Resource::Metal]) {
            return false;
        }

        tile.activated = false;
        tile.activation_resources = vec![];

        let x = tile.additional_data.get("nuke_target_x");
        let y = tile.additional_data.get("nuke_target_y");

        let x: i32 = match x {
            Some(x) => x.parse().unwrap(),
            None => return false,
        };

        let y: i32 = match y {
            Some(y) => y.parse().unwrap(),
            None => return false,
        };
        
        let tiles_to_destroy = game_state.map.get_tiles_in_range(x, y, 3);
        for (_, tile) in tiles_to_destroy {
            if let TileOption::Id(id) = tile {
                game_state.tiles.remove(&id);
                game_state.map.remove_tile(id);
            }
        }
        true

    }
}

impl ActiveAbility for TechMarketActive {
    fn trigger(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        let mut trigger_mode = 0;
        if self.can_trigger(game_state, tile, &vec![Resource::Gold]) {
            trigger_mode = 1;
        }
        if self.can_trigger(game_state, tile, &vec![Resource::Metal]) {
            trigger_mode = 2;
        }
        if trigger_mode == 0 {
            return false;
        }

        tile.activated = false;
        tile.activation_resources = vec![];
            
        // deconstruct building
        if trigger_mode == 1 {
            let deconstruct_tile_id = tile.additional_data.remove("deconstruct_id");
            let deconstruct_tile_id = match deconstruct_tile_id {
                Some(id) => id,
                None => return false,
            };

            game_state.tiles.remove(&deconstruct_tile_id);
            return game_state.map.remove_tile(deconstruct_tile_id);
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
    fn trigger(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !self.can_trigger(game_state, tile, &vec![Resource::Gold]) {
            return false;
        }

        tile.activated = false;
        tile.activation_resources = vec![];
        game_state.tech_resources.push(Resource::Metal);
        game_state.tech_resources.push(Resource::Metal);

        true
    }
}

impl ActiveAbility for TechRefinery1Active {
    fn trigger(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !self.can_trigger(game_state, tile, &vec![Resource::Gold]) {
            return false;
        }
        tile.activated = false;
        tile.activation_resources = vec![];
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
