use std::collections::HashMap;

use serde::Serialize;

use crate::game::game_models::types::tile::TileSelector;




#[derive(Debug, Serialize)]
pub enum GameCommand {
    GetState,
    BaseSetup(i32, i32),
    PlaceTile(TileSelector, i32, i32, i32),
    ActivateAbility(String, i32, HashMap<String, String>),
    NextPhase,
    ApplyPhase,
    GetRecepies,
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
            "PlaceTile" => {
                if tokens.len() != 5 {
                    Self::InvalidCommand("5 tokens needed ".to_string())
                } else {
                    let unit_selector = match parse_tile_selector(tokens[1]) {
                        Some(s) => s,
                        None => return Self::InvalidCommand("Can't parse unit selector".to_string()),
                    };
                    let x: i32 = match tokens[2].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse X".to_string()),
                    };
                    let y: i32 = match tokens[3].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse Y".to_string()),
                    };
                    let rotat: i32 = match tokens[4].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse rotation".to_string()),
                    };
                    Self::PlaceTile(unit_selector, x, y, rotat)
                }
            },
            "ActivateAbility" => {
                if tokens.len() < 3 {
                    Self::InvalidCommand("At least 3 tokens needed for ActivateAbility".to_string())
                } else {
                    let tile_id = tokens[1].to_string();
                    let ability_index: i32 = match tokens[2].parse() {
                        Ok(n) => n,
                        Err(_) => return Self::InvalidCommand("Can't parse incex value for ActivateAbility".to_string()),
                    };
                    let mut params = HashMap::new();
                    for param in tokens.iter().skip(3) {
                        let pair: Vec<&str> = param.split('=').collect();
                        if pair.len() == 2 {
                            params.insert(pair[0].to_string(), pair[1].to_string());
                        } else {
                            return Self::InvalidCommand("Invalid key=value pair for ActivateAbility".to_string());
                        }
                    }
                    Self::ActivateAbility(tile_id, ability_index, params)
                }
            },
            "NextPhase" => Self::NextPhase,
            "ApplyPhase" => Self::ApplyPhase,
            "GetRecepies" => Self::GetRecepies,
            "Undo" => Self::Undo,
            _ => Self::InvalidCommand("Command not found".to_string()),
        }
    }
}

fn parse_tile_selector(token: &str) -> Option<TileSelector> {
    match token {
        "BugBase1" => Some(TileSelector::BugBase1),
        "BugBase2" => Some(TileSelector::BugBase2),
        "BugBase3" => Some(TileSelector::BugBase3),
        "TechBase" => Some(TileSelector::TechBase),
        "TechRoad" => Some(TileSelector::TechRoad),
        "TechMine1" => Some(TileSelector::TechMine1),
        "TechMine2" => Some(TileSelector::TechMine2),
        "TechRefinery1" => Some(TileSelector::TechRefinery1),
        "TechRefinery2" => Some(TileSelector::TechRefinery2),
        "TechMarket" => Some(TileSelector::TechMarket),
        "TechTurret1" => Some(TileSelector::TechTurret1),
        "TechTurret2" => Some(TileSelector::TechTurret2),
        "TechArtillery1" => Some(TileSelector::TechArtillery1),
        "TechArtillery2" => Some(TileSelector::TechArtillery2),
        "TechWall1" => Some(TileSelector::TechWall1),
        "TechNuke" => Some(TileSelector::TechNuke),
        "BugSoldierLV1" => Some(TileSelector::BugSoldierLV1),
        "BugSoldierLV2" => Some(TileSelector::BugSoldierLV2),
        "BugSoldierLV3" => Some(TileSelector::BugSoldierLV3),
        "BugEliteMelee" => Some(TileSelector::BugEliteMelee),
        "BugEliteRanged" => Some(TileSelector::BugEliteRanged),
        _ => None,
    }
}