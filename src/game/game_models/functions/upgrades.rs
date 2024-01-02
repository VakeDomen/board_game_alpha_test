use std::collections::HashMap;

use crate::game::{game_models::types::{structure::{StructureSelector, Structure}, tile_traits::Upgradable, resource::Resouce, map::Extrcator}, core::game_state::GameState};



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
        true
    }

    fn can_upgrade(&self, game_state: &GameState, structure: &mut Structure) -> bool {
        false
    }
}

impl Upgradable for BugBase1Upgrader {
    fn upgrade(&self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        if !&self.can_upgrade(game_state, structure) {
            return false;
        }
        structure.structure_type = StructureSelector::BugBase2;
        true
    }

    fn can_upgrade(&self, game_state: &GameState, structure: &mut Structure) -> bool {
        false
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

        if structure.activation_resources != vec![Resouce::Metal] {
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

        if structure.activation_resources != vec![Resouce::Metal] {
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

        if structure.activation_resources != vec![Resouce::Metal] {
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

        if structure.activation_resources != vec![Resouce::Metal] {
            return false;
        }
        true
    }
}