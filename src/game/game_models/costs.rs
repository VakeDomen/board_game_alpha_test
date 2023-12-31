use std::collections::HashMap;

use super::{structure::StructureSelector, resource::Resouce};

pub fn get_costs() -> HashMap<StructureSelector, Vec<Resouce>> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, vec![]);
    hm.insert(StructureSelector::BugBase2, vec![]);
    hm.insert(StructureSelector::BugBase3, vec![]);
    hm.insert(StructureSelector::TechBase, vec![]);
    hm.insert(StructureSelector::TechRoad, vec![
        Resouce::Gold, 
    ]);
    hm.insert(StructureSelector::TechMine1, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,

    ]);
    hm.insert(StructureSelector::TechMine2, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechRefinery1, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
    ]);
    hm.insert(StructureSelector::TechRefinery2, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechMarket, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechTurret1, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechTurret2, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechArtillery1, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechArtillery2, vec![
        Resouce::Gold, 
        Resouce::Gold, 
        Resouce::Gold,
        Resouce::Metal,
        Resouce::Metal,
    ]);
    hm.insert(StructureSelector::TechWall1, vec![
        Resouce::Gold, 
    ]);
    hm
}