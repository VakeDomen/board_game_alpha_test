use std::collections::HashMap;

use crate::game::game_models::types::tile::TileSelector;



pub fn get_footprints() -> HashMap<TileSelector, Vec<Vec<bool>>> {
    let mut hm = HashMap::new();
    hm.insert(TileSelector::BugBase1, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::BugBase2, vec![
        vec![false, true, false],
        vec![true, true, true],
        vec![false, true, false],
    ]);
    hm.insert(TileSelector::BugBase3, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm.insert(TileSelector::TechBase, vec![
        vec![true, true, true],
        vec![true, false, true],
        vec![true, true, true],
    ]);
    hm.insert(TileSelector::TechRoad, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechMine1, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(TileSelector::TechMine2, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(TileSelector::TechRefinery1, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(TileSelector::TechRefinery2, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(TileSelector::TechMarket, vec![
        vec![true, true],
        vec![true, true],
    ]);
    hm.insert(TileSelector::TechTurret1, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechTurret2, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechArtillery1, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechArtillery2, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechWall1, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::TechNuke, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm.insert(TileSelector::BugSoldierLV1, vec![
        vec![true]
    ]);
    hm.insert(TileSelector::BugSoldierLV2, vec![
        vec![true, true],
    ]);
    hm.insert(TileSelector::BugSoldierLV3, vec![
        vec![true, true, true],
    ]);
    hm.insert(TileSelector::BugEliteMelee, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm.insert(TileSelector::BugEliteRanged, vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);
    hm
}