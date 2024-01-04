use std::collections::HashMap;

use crate::game::{game_models::types::{tile::{TileSelector, Tile}, tile_traits::PassiveAbility, resource::Resource, map::Interactor}, core::{game_state::GameState, game::Player}};


pub struct BugBase1Passive;
pub struct BugBase2Passive;
pub struct BugBase3Passive;
pub struct TechBasePassive;
pub struct TechMine1Passive;
pub struct TechMine2Passive;


pub fn get_passive_abilities() -> HashMap<TileSelector, Option<Box<dyn PassiveAbility>>> {
    let mut hm: HashMap<TileSelector, Option<Box<dyn PassiveAbility>>> = HashMap::new();
    hm.insert(TileSelector::BugBase1, Some(Box::new(BugBase1Passive{})));
    hm.insert(TileSelector::BugBase2, Some(Box::new(BugBase2Passive{})));
    hm.insert(TileSelector::BugBase3, Some(Box::new(BugBase3Passive{})));
    hm.insert(TileSelector::TechBase, Some(Box::new(TechBasePassive{})));
    hm.insert(TileSelector::TechRoad, None);
    hm.insert(TileSelector::TechMine1, Some(Box::new(TechMine1Passive{})));
    hm.insert(TileSelector::TechMine2, Some(Box::new(TechMine2Passive{})));
    hm.insert(TileSelector::TechRefinery1, None);
    hm.insert(TileSelector::TechRefinery2, None);
    hm.insert(TileSelector::TechMarket, None);
    hm.insert(TileSelector::TechTurret1, None);
    hm.insert(TileSelector::TechTurret2, None);
    hm.insert(TileSelector::TechArtillery1, None);
    hm.insert(TileSelector::TechArtillery2, None);
    hm.insert(TileSelector::TechWall1, None);
    hm.insert(TileSelector::TechNuke, None);
    hm
}

impl PassiveAbility for TechBasePassive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        game_state.tech_resources.push(Resource::Gold);
        game_state.tech_resources.push(Resource::Gold);
        game_state.tech_resources.push(Resource::Gold);
        true
    }
}

impl PassiveAbility for TechMine1Passive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        game_state.tech_resources.push(Resource::Gold);
        game_state.tech_resources.push(Resource::Gold);
        true
    }
}

impl PassiveAbility for TechMine2Passive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        game_state.tech_resources.push(Resource::Gold);
        game_state.tech_resources.push(Resource::Gold);
        game_state.tech_resources.push(Resource::Gold);
        true
    }
}

impl PassiveAbility for BugBase1Passive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        let footprint = game_state.map.get_footprint_tiles_by_id(&tile.id);
        let mut on_bottom_edge = false;
        for (loc, _) in footprint {
            if loc.0 == game_state.map.len() - 1 {
                on_bottom_edge = true;
            }
        }

        if on_bottom_edge {
            game_state.bug_resources.push(Resource::Egg);
        }
        game_state.bug_resources.push(Resource::Egg);
        true
    }
}

impl PassiveAbility for BugBase2Passive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        let footprint = game_state.map.get_footprint_tiles_by_id(&tile.id);
        let mut on_bottom_edge = false;
        for (loc, _) in footprint {
            if loc.0 == game_state.map.len() - 1 {
                on_bottom_edge = true;
            }
        }

        if on_bottom_edge {
            game_state.bug_resources.push(Resource::Egg);
        }

        game_state.bug_resources.push(Resource::Egg);
        game_state.bug_resources.push(Resource::Egg);
        true
    }
}

impl PassiveAbility for BugBase3Passive {
    fn activate_passive(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if game_state.player_turn != tile.owner {
            return false;
        }
        let footprint = game_state.map.get_footprint_tiles_by_id(&tile.id);
        let mut on_bottom_edge = false;
        for (loc, _) in footprint {
            if loc.0 == game_state.map.len() - 1 {
                on_bottom_edge = true;
            }
        }

        if on_bottom_edge {
            game_state.bug_resources.push(Resource::Egg);
        }

        game_state.bug_resources.push(Resource::Egg);
        game_state.bug_resources.push(Resource::Egg);
        game_state.bug_resources.push(Resource::Egg);
        true
    }
}


