use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game::game_models::types::tile::TileSelector;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Move {
    Tech(TechMove),
    Bug(BugMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TechMove {
    SetupMove(i32, i32),
    DmgMove(String, String, i32), // initiator id, target id, dmg count
    MainMove(TechMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TechMainPhaseMove {
    Build(TileSelector, i32, i32),
    ActivateAbility(String, i32, HashMap<String, String>), // id, index of ability(usually 0), additional data
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BugMove {
    SetupMove(i32, i32),
    DmgMove(String, String, i32), // initiator id, target id, dmg count
    MainMove(BugMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BugMainPhaseMove {
    PlaceTile(TileSelector, i32, i32, i32), // selector, x, y, orientation (0-3)
    ActivateAbility(String, i32, HashMap<String, String>), // id and index of ability. usually 0
    SacrificeTile(String),
}