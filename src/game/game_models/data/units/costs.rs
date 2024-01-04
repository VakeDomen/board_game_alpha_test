use std::collections::HashMap;

use crate::game::game_models::types::{resource::Resource, unit::UnitSelector};


pub fn get_costs() -> HashMap<UnitSelector, Vec<Resource>> {
    let mut hm = HashMap::new();
    hm.insert(UnitSelector::BugSoldierLV1, vec![
        Resource::Egg, 
    ]);
    hm.insert(UnitSelector::BugSoldierLV2, vec![
        Resource::Egg, 
        Resource::Egg, 
    ]);
    hm.insert(UnitSelector::BugSoldierLV3, vec![
        Resource::Egg, 
        Resource::Egg, 
        Resource::Egg, 
    ]);
    hm.insert(UnitSelector::BugEliteMelee, vec![]);
    hm.insert(UnitSelector::BugEliteRanged, vec![]);
    
    hm
}