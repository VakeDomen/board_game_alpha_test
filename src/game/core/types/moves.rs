use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game::game_models::types::tile::TileSelector;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Move {
    Tech(PhaseMove),
    Bug(PhaseMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhaseMove {
    SetupMove(i32, i32),
    DmgMove(String, String, i32), // initiator id, target id, dmg count
    MainMove(MainPhaseMove),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MainPhaseMove {
    PlaceTile(TileSelector, i32, i32, i32), // selector, x, y, orientation (0-3)
    ActivateAbility(String, i32, HashMap<String, String>), // id and index of ability. usually 0
    SacrificeTile(String),
}