use std::collections::HashMap;

use crate::game::game_models::types::structure::StructureSelector;


pub fn get_footprints() -> HashMap<StructureSelector, Vec<Vec<bool>>> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::BugBase2, vec![
        vec![false, true, false],
        vec![true, true, true],
        vec![false, true, false],
    ]);
    hm.insert(StructureSelector::BugBase3, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm.insert(StructureSelector::TechBase, vec![
        vec![true, true, true],
        vec![true, false, true],
        vec![true, true, true],
    ]);
    hm.insert(StructureSelector::TechRoad, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechMine1, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(StructureSelector::TechMine2, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(StructureSelector::TechRefinery1, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(StructureSelector::TechRefinery2, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(StructureSelector::TechMarket, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(StructureSelector::TechTurret1, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechTurret2, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechArtillery1, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechArtillery2, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechWall1, vec![
        vec![true]
    ]);
    hm.insert(StructureSelector::TechNuke, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm
}