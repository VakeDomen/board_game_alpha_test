use serde::{Serialize, Deserialize};

use crate::game::game_models::types::{structure::StructureSelector, unit::UnitSelector};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Move {
    Tech(TechMove),
    Bug(BugMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TechMove {
    SetupMove(i32, i32),
    DmgMove(i32, i32, i32), // initiator id, target id, dmg count
    MainMove(TechMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TechMainPhaseMove {
    Build(StructureSelector, i32, i32),
    ActivateAbility(String, i32), // id and index of ability. usually 0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BugMove {
    SetupMove(i32, i32),
    DmgMove(i32, i32, i32), // initiator id, target id, dmg count
    MainMove(BugMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BugMainPhaseMove {
    PlaceUnit(UnitSelector, i32, i32, i32), // selector, x, y, orientation (0-3)
    ActivateAbility(String, i32), // id and index of ability. usually 0
    SacrificeUnit(String),
}