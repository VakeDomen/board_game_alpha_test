use std::collections::HashMap;

use crate::game::game_models::types::{resource::Resouce, unit::UnitSelector};


pub fn get_costs() -> HashMap<UnitSelector, Vec<Resouce>> {
    let mut hm = HashMap::new();
    hm.insert(UnitSelector::BugSoldierLV1, vec![
        Resouce::Egg, 
    ]);
    hm.insert(UnitSelector::BugSoldierLV2, vec![
        Resouce::Egg, 
        Resouce::Egg, 
    ]);
    hm.insert(UnitSelector::BugSoldierLV3, vec![
        Resouce::Egg, 
        Resouce::Egg, 
        Resouce::Egg, 
    ]);
    hm.insert(UnitSelector::BugEliteMelee, vec![]);
    hm.insert(UnitSelector::BugEliteRanged, vec![]);
    
    hm
}