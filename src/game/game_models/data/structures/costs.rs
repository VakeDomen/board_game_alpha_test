use std::collections::HashMap;

use crate::game::game_models::types::{resource::Resource, structure::StructureSelector};


pub fn get_costs() -> HashMap<StructureSelector, Vec<Resource>> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, vec![]);
    hm.insert(StructureSelector::BugBase2, vec![]);
    hm.insert(StructureSelector::BugBase3, vec![]);
    hm.insert(StructureSelector::TechBase, vec![]);
    hm.insert(StructureSelector::TechRoad, vec![
        Resource::Gold, 
    ]);
    hm.insert(StructureSelector::TechMine1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,

    ]);
    hm.insert(StructureSelector::TechMine2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechRefinery1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
    ]);
    hm.insert(StructureSelector::TechRefinery2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechMarket, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechTurret1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechTurret2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechArtillery1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechArtillery2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
        Resource::Metal,
    ]);
    hm.insert(StructureSelector::TechWall1, vec![
        Resource::Gold, 
    ]);
    hm.insert(StructureSelector::TechNuke, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Metal, 
        Resource::Metal, 
        Resource::Metal, 
        Resource::Metal, 
        Resource::Metal, 
    ]);
    hm
}