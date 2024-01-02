use std::collections::HashMap;

use crate::game::game_models::types::unit::UnitSelector;


pub fn get_footprints() -> HashMap<UnitSelector, Vec<Vec<bool>>> {
    let mut hm = HashMap::new();
    hm.insert(UnitSelector::BugSoldierLV1, vec![
        vec![true]
    ]);
    hm.insert(UnitSelector::BugSoldierLV2, vec![
        vec![true, true],
    ]);
    hm.insert(UnitSelector::BugSoldierLV3, vec![
        vec![true, true, true],
    ]);
    hm.insert(UnitSelector::BugEliteMelee, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm.insert(UnitSelector::BugEliteRanged, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm
}