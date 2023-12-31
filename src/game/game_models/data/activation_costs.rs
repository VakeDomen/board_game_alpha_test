use std::collections::HashMap;

use crate::game::game_models::types::{structure::StructureSelector, resource::Resouce};


pub fn get_activation_costs() -> HashMap<StructureSelector, Vec<Vec<Resouce>>> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, vec![],);
    hm.insert(StructureSelector::BugBase2, vec![],);
    hm.insert(StructureSelector::BugBase3, vec![],);
    hm.insert(StructureSelector::TechBase, vec![]);
    hm.insert(StructureSelector::TechRoad, vec![]);
    hm.insert(StructureSelector::TechMine1, vec![
        vec![Resouce::Metal]
    ]);
    hm.insert(StructureSelector::TechMine2, vec![]);
    hm.insert(StructureSelector::TechRefinery1, vec![
        vec![Resouce::Metal],
        vec![Resouce::Gold]
    ]);
    hm.insert(StructureSelector::TechRefinery2, vec![
        vec![Resouce::Gold]
    ]);
    hm.insert(StructureSelector::TechMarket, vec![
        vec![Resouce::Gold],
        vec![Resouce::Metal]
    ]);
    hm.insert(StructureSelector::TechTurret1, vec![
        vec![Resouce::Metal],
    ]);
    hm.insert(StructureSelector::TechTurret2, vec![]);
    hm.insert(StructureSelector::TechArtillery1, vec![
        vec![Resouce::Metal],
    ]);
    hm.insert(StructureSelector::TechArtillery2, vec![]);
    hm.insert(StructureSelector::TechWall1, vec![]);
    
    hm
}