use std::collections::HashMap;

use crate::game::game_models::types::{tile::TileSelector, resource::Resource};


pub fn get_costs() -> HashMap<TileSelector, Vec<Resource>> {
    let mut hm = HashMap::new();
    hm.insert(TileSelector::BugBase1, vec![]);
    hm.insert(TileSelector::BugBase2, vec![]);
    hm.insert(TileSelector::BugBase3, vec![]);
    hm.insert(TileSelector::TechBase, vec![]);
    hm.insert(TileSelector::TechRoad, vec![
        Resource::Gold, 
    ]);
    hm.insert(TileSelector::TechMine1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,

    ]);
    hm.insert(TileSelector::TechMine2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechRefinery1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
    ]);
    hm.insert(TileSelector::TechRefinery2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechMarket, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechTurret1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechTurret2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechArtillery1, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechArtillery2, vec![
        Resource::Gold, 
        Resource::Gold, 
        Resource::Gold,
        Resource::Metal,
        Resource::Metal,
    ]);
    hm.insert(TileSelector::TechWall1, vec![
        Resource::Gold, 
    ]);
    hm.insert(TileSelector::TechNuke, vec![
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
    hm.insert(TileSelector::BugSoldierLV1, vec![
        Resource::Egg, 
    ]);
    hm.insert(TileSelector::BugSoldierLV2, vec![
        Resource::Egg, 
        Resource::Egg, 
    ]);
    hm.insert(TileSelector::BugSoldierLV3, vec![
        Resource::Egg, 
        Resource::Egg, 
        Resource::Egg, 
    ]);
    hm.insert(TileSelector::BugEliteMelee, vec![
        Resource::GiantEgg
    ]);
    hm.insert(TileSelector::BugEliteRanged, vec![
        Resource::GiantEgg
    ]);
    hm
}