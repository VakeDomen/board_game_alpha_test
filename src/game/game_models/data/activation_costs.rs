use std::{collections::HashMap, vec};

use crate::game::game_models::types::{tile::TileSelector, resource::Resource};



pub fn get_activation_costs() -> HashMap<TileSelector, Vec<Vec<Resource>>> {
    let mut hm = HashMap::new();
    hm.insert(TileSelector::BugBase1, vec![],);
    hm.insert(TileSelector::BugBase2, vec![],);
    hm.insert(TileSelector::BugBase3, vec![
        vec![
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
        ]
    ],);
    hm.insert(TileSelector::TechBase, vec![]);
    hm.insert(TileSelector::TechRoad, vec![]);
    hm.insert(TileSelector::TechMine1, vec![
        vec![Resource::Metal]
    ]);
    hm.insert(TileSelector::TechMine2, vec![]);
    hm.insert(TileSelector::TechRefinery1, vec![
        vec![Resource::Metal],
        vec![Resource::Gold]
    ]);
    hm.insert(TileSelector::TechRefinery2, vec![
        vec![Resource::Gold]
    ]);
    hm.insert(TileSelector::TechMarket, vec![
        vec![Resource::Gold],
        vec![Resource::Metal]
    ]);
    hm.insert(TileSelector::TechTurret1, vec![
        vec![Resource::Metal],
    ]);
    hm.insert(TileSelector::TechTurret2, vec![]);
    hm.insert(TileSelector::TechArtillery1, vec![
        vec![Resource::Metal],
    ]);
    hm.insert(TileSelector::TechArtillery2, vec![]);
    hm.insert(TileSelector::TechWall1, vec![]);
    hm.insert(TileSelector::TechNuke, vec![
        vec![Resource::Metal, Resource::Metal, Resource::Metal],
    ]);
    hm.insert(TileSelector::BugSoldierLV1, vec![]);
    hm.insert(TileSelector::BugSoldierLV2, vec![]);
    hm.insert(TileSelector::BugSoldierLV3, vec![]);
    hm.insert(TileSelector::BugEliteMelee, vec![
        vec![Resource::Corpse, Resource::Corpse, Resource::Corpse]
    ]);
    hm.insert(TileSelector::BugEliteRanged, vec![
        vec![Resource::Corpse, Resource::Corpse, Resource::Corpse]
    ]);

    hm
}