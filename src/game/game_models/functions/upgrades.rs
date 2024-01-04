use std::collections::HashMap;

use crate::game::{game_models::types::{structure::{StructureSelector, Structure}, tile_traits::{Upgradable, Tile}, resource::Resource, map::{Interactor, TileOption}, unit::UnitSelector}, core::game_state::GameState};



pub struct TurretUpgrader;
pub struct AtrileryUpgrader;
pub struct MineUpgrader;
pub struct RefineryUpgrader;
pub struct BugBase1Upgrader;
pub struct BugBase2Upgrader;


pub fn get_upgraders() -> HashMap<StructureSelector, Option<Box<dyn Upgradable>>> {
    let mut hm: HashMap<StructureSelector, Option<Box<dyn Upgradable>>> = HashMap::new();
    hm.insert(StructureSelector::BugBase1, Some(Box::new(BugBase1Upgrader{})));
    hm.insert(StructureSelector::BugBase2, Some(Box::new(BugBase2Upgrader{})));
    hm.insert(StructureSelector::BugBase3, None);
    hm.insert(StructureSelector::TechBase, None);
    hm.insert(StructureSelector::TechRoad, None);
    hm.insert(StructureSelector::TechMine1, Some(Box::new(MineUpgrader{})));
    hm.insert(StructureSelector::TechMine2, None);
    hm.insert(StructureSelector::TechRefinery1, Some(Box::new(RefineryUpgrader{})));
    hm.insert(StructureSelector::TechRefinery2, None);
    hm.insert(StructureSelector::TechMarket, None);
    hm.insert(StructureSelector::TechTurret1, Some(Box::new(TurretUpgrader{})));
    hm.insert(StructureSelector::TechTurret2, None);
    hm.insert(StructureSelector::TechArtillery1, Some(Box::new(AtrileryUpgrader{})));
    hm.insert(StructureSelector::TechArtillery2, None);
    hm.insert(StructureSelector::TechWall1, None);
    hm.insert(StructureSelector::TechNuke, None);
    hm
}


impl Upgradable for BugBase2Upgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.structure_type = StructureSelector::BugBase3;
        for (location, tile_option) in game_state.map.get_tile_corners(structure.x, structure.y) {
            if let TileOption::Id(id) = tile_option {
                game_state.tiles.remove(&id);
            }
            game_state.map[location.0][location.1] = structure.id.clone();
        }

        true
    }

    fn can_upgrade(&self, game_state: &GameState, structure: &mut Structure) -> bool {
        for (_, tile_option) in game_state.map.get_tile_corners(structure.x + 1, structure.y + 1) {
            let tile = match tile_option {
                TileOption::Id(id) => game_state.tiles.get(&id).unwrap(),
                TileOption::None => return false,
                TileOption::OutOfBounds => return false,
            };
            let unit = match tile {
                Tile::Structure(_) => return false,
                Tile::Unit(u) => u,
            };
            if UnitSelector::BugSoldierLV1 != unit.unit_type {
                return false
            }
        }
        true
    }
}

impl Upgradable for BugBase1Upgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.structure_type = StructureSelector::BugBase2;

        for (location, tile_option) in game_state.map.get_tile_adjacent(structure.x, structure.y) {
            if let TileOption::Id(id) = tile_option {
                game_state.tiles.remove(&id);
            }
            game_state.map[location.0][location.1] = structure.id.clone();
        }
        // offset top left corner
        structure.x -= 1;
        structure.y -= 1;

        true
    }

    fn can_upgrade(&self, game_state: &GameState, structure: &mut Structure) -> bool {
        for (_, tile_option) in game_state.map.get_tile_adjacent(structure.x, structure.y) {
            let tile = match tile_option {
                TileOption::Id(id) => game_state.tiles.get(&id).unwrap(),
                TileOption::None => return false,
                TileOption::OutOfBounds => return false,
            };
            let unit = match tile {
                Tile::Structure(_) => return false,
                Tile::Unit(u) => u,
            };
            if UnitSelector::BugSoldierLV1 != unit.unit_type {
                return false
            }
        }
        true
    }
}

impl Upgradable for RefineryUpgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.activated = false;
        structure.activation_resources = vec![];
        structure.structure_type = StructureSelector::TechRefinery2;
        true
    }

    fn can_upgrade(&self, _: &GameState, structure: &mut Structure) -> bool {
        if !structure.activated {
            return false
        }

        if structure.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for MineUpgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.activated = false;
        structure.activation_resources = vec![];
        structure.structure_type = StructureSelector::TechMine2;
        true
    }

    fn can_upgrade(&self, _: &GameState, structure: &mut Structure) -> bool {
        if !structure.activated {
            return false
        }

        if structure.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for TurretUpgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.activated = false;
        structure.activation_resources = vec![];
        structure.structure_type = StructureSelector::TechTurret2;
        true
    }

    fn can_upgrade(&self, _: &GameState, structure: &mut Structure) -> bool {
        if !structure.activated {
            return false
        }

        if structure.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}

impl Upgradable for AtrileryUpgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.activated = false;
        structure.activation_resources = vec![];
        structure.structure_type = StructureSelector::TechArtillery2;
        true
    }

    fn can_upgrade(&self, _: &GameState, structure: &mut Structure) -> bool {
        if !structure.activated {
            return false
        }

        if structure.activation_resources != vec![Resource::Metal] {
            return false;
        }
        true
    }
}