use std::collections::HashMap;

use crate::game::{game_models::types::{tile::{TileSelector, Tile}, tile_traits::Upgradable, map::{Interactor, TileOption}, resource::Resource}, core::game_state::GameState};


pub struct TurretUpgrader;
pub struct AtrileryUpgrader;
pub struct MineUpgrader;
pub struct RefineryUpgrader;
pub struct BugBase1Upgrader;
pub struct BugBase2Upgrader;


pub fn get_upgraders() -> HashMap<TileSelector, Option<Box<dyn Upgradable>>> {
    let mut hm: HashMap<TileSelector, Option<Box<dyn Upgradable>>> = HashMap::new();
    hm.insert(TileSelector::BugBase1, Some(Box::new(BugBase1Upgrader{})));
    hm.insert(TileSelector::BugBase2, Some(Box::new(BugBase2Upgrader{})));
    hm.insert(TileSelector::BugBase3, None);
    hm.insert(TileSelector::TechBase, None);
    hm.insert(TileSelector::TechRoad, None);
    hm.insert(TileSelector::TechMine1, Some(Box::new(MineUpgrader{})));
    hm.insert(TileSelector::TechMine2, None);
    hm.insert(TileSelector::TechRefinery1, Some(Box::new(RefineryUpgrader{})));
    hm.insert(TileSelector::TechRefinery2, None);
    hm.insert(TileSelector::TechMarket, None);
    hm.insert(TileSelector::TechTurret1, Some(Box::new(TurretUpgrader{})));
    hm.insert(TileSelector::TechTurret2, None);
    hm.insert(TileSelector::TechArtillery1, Some(Box::new(AtrileryUpgrader{})));
    hm.insert(TileSelector::TechArtillery2, None);
    hm.insert(TileSelector::TechWall1, None);
    hm.insert(TileSelector::TechNuke, None);
    hm
}


impl Upgradable for BugBase2Upgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.tile_type = TileSelector::BugBase3;
        for (location, tile_option) in game_state.map.get_tile_corners(tile.x, tile.y) {
            if let TileOption::Id(id) = tile_option {
                game_state.tiles.remove(&id);
            }
            game_state.map[location.0][location.1] = tile.id.clone();
        }

        true
    }

    fn can_upgrade(&self, game_state: &GameState, tile: &mut Tile) -> bool {
        for (_, tile_option) in game_state.map.get_tile_corners(tile.x + 1, tile.y + 1) {
            let tile = match tile_option {
                TileOption::Id(id) => game_state.tiles.get(&id).unwrap(),
                TileOption::None => return false,
                TileOption::OutOfBounds => return false,
            };
            if TileSelector::BugSoldierLV1 != tile.tile_type {
                return false
            }
        }
        true
    }
}

impl Upgradable for BugBase1Upgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.tile_type = TileSelector::BugBase2;

        for (location, tile_option) in game_state.map.get_tile_adjacent(tile.x, tile.y) {
            if let TileOption::Id(id) = tile_option {
                game_state.tiles.remove(&id);
            }
            game_state.map[location.0][location.1] = tile.id.clone();
        }
        // offset top left corner
        tile.x -= 1;
        tile.y -= 1;

        true
    }

    fn can_upgrade(&self, game_state: &GameState, tile: &mut Tile) -> bool {
        for (_, tile_option) in game_state.map.get_tile_adjacent(tile.x, tile.y) {
            let tile = match tile_option {
                TileOption::Id(id) => game_state.tiles.get(&id).unwrap(),
                TileOption::None => return false,
                TileOption::OutOfBounds => return false,
            };
            if TileSelector::BugSoldierLV1 != tile.tile_type {
                return false
            }
        }
        true
    }
}

impl Upgradable for RefineryUpgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.activated = false;
        tile.activation_resources = vec![];
        tile.tile_type = TileSelector::TechRefinery2;
        true
    }

    fn can_upgrade(&self, _: &GameState, tile: &mut Tile) -> bool {
        if !tile.activated {
            return false
        }

        if tile.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for MineUpgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.activated = false;
        tile.activation_resources = vec![];
        tile.tile_type = TileSelector::TechMine2;
        true
    }

    fn can_upgrade(&self, _: &GameState, tile: &mut Tile) -> bool {
        if !tile.activated {
            return false
        }

        if tile.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for TurretUpgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.activated = false;
        tile.activation_resources = vec![];
        tile.tile_type = TileSelector::TechTurret2;
        true
    }

    fn can_upgrade(&self, _: &GameState, tile: &mut Tile) -> bool {
        if !tile.activated {
            return false
        }

        if tile.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for AtrileryUpgrader {
    fn upgrade(&self, game_state: &mut GameState, tile: &mut Tile) -> bool {
        if !&self.can_upgrade(game_state, tile) {
            return false;
        }
        tile.activated = false;
        tile.activation_resources = vec![];
        tile.tile_type = TileSelector::TechArtillery2;
        true
    }

    fn can_upgrade(&self, _: &GameState, tile: &mut Tile) -> bool {
        if !tile.activated {
            return false
        }

        if tile.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}