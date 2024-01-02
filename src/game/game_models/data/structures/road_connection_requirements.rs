use std::collections::HashMap;

use crate::game::game_models::types::structure::StructureSelector;


pub fn get_road_connection_requirements() -> HashMap<StructureSelector, bool> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, false);
    hm.insert(StructureSelector::BugBase2, false);
    hm.insert(StructureSelector::BugBase3, false);
    hm.insert(StructureSelector::TechBase, false);
    hm.insert(StructureSelector::TechRoad, true);
    hm.insert(StructureSelector::TechMine1, true);
    hm.insert(StructureSelector::TechMine2, true);
    hm.insert(StructureSelector::TechRefinery1, true);
    hm.insert(StructureSelector::TechRefinery2, true);
    hm.insert(StructureSelector::TechMarket, true);
    hm.insert(StructureSelector::TechTurret1,true);
    hm.insert(StructureSelector::TechTurret2, true);
    hm.insert(StructureSelector::TechArtillery1, true);
    hm.insert(StructureSelector::TechArtillery2, true);
    hm.insert(StructureSelector::TechWall1, false);
    hm.insert(StructureSelector::TechNuke, false);

    hm
}