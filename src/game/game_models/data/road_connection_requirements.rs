use std::collections::HashMap;

use crate::game::game_models::types::tile::TileSelector;


pub fn get_road_connection_requirements() -> HashMap<TileSelector, bool> {
    let mut hm = HashMap::new();
    hm.insert(TileSelector::BugBase1, false);
    hm.insert(TileSelector::BugBase2, false);
    hm.insert(TileSelector::BugBase3, false);
    hm.insert(TileSelector::TechBase, false);
    hm.insert(TileSelector::TechRoad, true);
    hm.insert(TileSelector::TechMine1, true);
    hm.insert(TileSelector::TechMine2, true);
    hm.insert(TileSelector::TechRefinery1, true);
    hm.insert(TileSelector::TechRefinery2, true);
    hm.insert(TileSelector::TechMarket, true);
    hm.insert(TileSelector::TechTurret1,true);
    hm.insert(TileSelector::TechTurret2, true);
    hm.insert(TileSelector::TechArtillery1, true);
    hm.insert(TileSelector::TechArtillery2, true);
    hm.insert(TileSelector::TechWall1, false);
    hm.insert(TileSelector::TechNuke, false);
    hm.insert(TileSelector::BugSoldierLV1, false);
    hm.insert(TileSelector::BugSoldierLV2, false);
    hm.insert(TileSelector::BugSoldierLV3, false);
    hm.insert(TileSelector::BugEliteMelee, false);
    hm.insert(TileSelector::BugEliteRanged, false);

    hm
}