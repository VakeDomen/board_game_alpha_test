use serde::Serialize;

use crate::game::game_models::types::structure::StructureSelector;



#[derive(Debug, Serialize)]
pub enum GameCommand {
    GetState,
    BaseSetup(i32, i32),
    PlaceStructure(StructureSelector, i32, i32),
    NextPhase,
    Undo,
    InvalidCommand(String),
}

impl From<String> for GameCommand {
    fn from(value: String) -> Self {
        let tokens: Vec<&str> = value.split(" ").collect();
        if tokens.len() == 0 {
            return Self::InvalidCommand("No tokens".to_string());
        }

        match tokens[0] {
            "GetState" => Self::GetState,
            "BaseSetup" => {
                if tokens.len() != 3 {
                    Self::InvalidCommand("3 tokens needed".to_string())
                } else {
                    let x: i32 = match tokens[1].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse X".to_string()),
                    };
                    let y: i32 = match tokens[2].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse Y".to_string()),
                    };
                    Self::BaseSetup(x, y)
                }
            },
            "PlaceStructure" => {
                if tokens.len() != 4 {
                    Self::InvalidCommand("4 tokens needed".to_string())
                } else {
                    let structure_selector = match parse_structure_selector(tokens[1]) {
                        Some(s) => s,
                        None => return Self::InvalidCommand("Can't parse structure selector".to_string()),
                    };
                    let x: i32 = match tokens[2].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse X".to_string()),
                    };
                    let y: i32 = match tokens[3].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse Y".to_string()),
                    };
                    Self::PlaceStructure(structure_selector, x, y)
                }
            }
            "NextPhase" => Self::NextPhase,
            "Undo" => Self::Undo,
            _ => Self::InvalidCommand("Command not found".to_string()),
        }
    }
}

fn parse_structure_selector(token: &str) -> Option<StructureSelector> {
    match token {
        "BugBase1" => Some(StructureSelector::BugBase1),
        "BugBase2" => Some(StructureSelector::BugBase2),
        "BugBase3" => Some(StructureSelector::BugBase3),
        "TechBase" => Some(StructureSelector::TechBase),
        "TechRoad" => Some(StructureSelector::TechRoad),
        "TechMine1" => Some(StructureSelector::TechMine1),
        "TechMine2" => Some(StructureSelector::TechMine2),
        "TechRefinery1" => Some(StructureSelector::TechRefinery1),
        "TechRefinery2" => Some(StructureSelector::TechRefinery2),
        "TechMarket" => Some(StructureSelector::TechMarket),
        "TechTurret1" => Some(StructureSelector::TechTurret1),
        "TechTurret2" => Some(StructureSelector::TechTurret2),
        "TechArtillery1" => Some(StructureSelector::TechArtillery1),
        "TechArtillery2" => Some(StructureSelector::TechArtillery2),
        "TechWall1" => Some(StructureSelector::TechWall1),
        "TechNuke" => Some(StructureSelector::TechNuke),
        _ => None,
    }
}