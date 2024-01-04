use std::collections::HashMap;

use serde::Serialize;

use crate::game::game_models::types::tile::TileSelector;



#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct TileStats {
    pub hp: i32,
    pub attack: i32,
    pub range: i32,
}

pub fn get_stats() -> HashMap<TileSelector, TileStats> {
    let mut hm = HashMap::new();
    hm.insert(TileSelector::BugBase1, TileStats {
        hp: 1,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::BugBase2, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::BugBase3, TileStats {
        hp: 3,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechBase, TileStats {
        hp: 10,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechMine1, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechMine2, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechRefinery1, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechRefinery2, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechMarket, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechTurret1, TileStats {
        hp: 3,
        attack: 3,
        range: 2,
    });
    hm.insert(TileSelector::TechTurret2, TileStats {
        hp: 3,
        attack: 5,
        range: 3,
    });
    hm.insert(TileSelector::TechArtillery1, TileStats {
        hp: 3,
        attack: 1,
        range: 7,
    });
    hm.insert(TileSelector::TechArtillery2, TileStats {
        hp: 3,
        attack: 2,
        range: 8,
    });
    hm.insert(TileSelector::TechWall1, TileStats {
        hp: 2,
        attack: 0,
        range: 0,
    });
    hm.insert(TileSelector::TechRoad, TileStats {
        hp: 1,
        attack: 0,
        range: 0,
    });

    hm.insert(TileSelector::TechNuke, TileStats {
        hp: 1,
        attack: 0,
        range: 0,
    });

    hm
}