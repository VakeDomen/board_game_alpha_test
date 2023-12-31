use std::collections::HashMap;

use serde::Serialize;

use crate::game::game_models::types::structure::StructureSelector;


#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct StructureStats {
    pub hp: i32,
    pub attack: i32,
    pub range: i32,
}

pub fn get_stats() -> HashMap<StructureSelector, StructureStats> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, StructureStats {
        hp: 1,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::BugBase2, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::BugBase3, StructureStats {
        hp: 3,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechBase, StructureStats {
        hp: 10,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechMine1, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechMine2, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechRefinery1, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechRefinery2, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechMarket, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechTurret1, StructureStats {
        hp: 3,
        attack: 3,
        range: 2,
    });
    hm.insert(StructureSelector::TechTurret2, StructureStats {
        hp: 3,
        attack: 5,
        range: 3,
    });
    hm.insert(StructureSelector::TechArtillery1, StructureStats {
        hp: 3,
        attack: 1,
        range: 7,
    });
    hm.insert(StructureSelector::TechArtillery2, StructureStats {
        hp: 3,
        attack: 2,
        range: 8,
    });
    hm.insert(StructureSelector::TechWall1, StructureStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(StructureSelector::TechRoad, StructureStats {
        hp: 1,
        attack: 0,
        range: 0,
    });

    hm
}