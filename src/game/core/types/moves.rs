use crate::game::game_models::types::{structure::StructureSelector, resource::Resouce};

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Tech(TechMove),
    Bug(BugMove),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TechMove {
    SetupMove(i32, i32),
    DmgMove(i32, i32, i32), // initiator id, target id, dmg count
    MainMove(TechMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TechMainPhaseMove {
    Build(StructureSelector, i32, i32),
    ActivateAbility(StructureSelector, Resouce),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BugMove {
    SetupMove(i32, i32),
    DmgMove(i32, i32, i32), // initiator id, target id, dmg count
    MainMove(BugMainPhaseMove),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BugMainPhaseMove {
    PlaceUnit(StructureSelector, i32, i32),
    ActivateAbility(StructureSelector, Resouce),
}