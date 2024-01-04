
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game::{game_models::data::stats::TileStats, core::game::Player};

use super::resource::Resource;


#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum TileSelector {
    BugBase1,
    BugBase2,
    BugBase3,
    TechBase,
    TechRoad,
    TechMine1,
    TechMine2,
    TechRefinery1,
    TechRefinery2,
    TechMarket,
    TechTurret1,
    TechTurret2,
    TechArtillery1,
    TechArtillery2,
    TechWall1,
    TechNuke,
    BugSoldierLV1,
    BugSoldierLV2,
    BugSoldierLV3,
    BugEliteMelee,
    BugEliteRanged,
}

#[derive(Debug, Clone)]
pub struct NewTile {
    pub owner: Player,
    pub tile_type: TileSelector,
    pub id: String,
    pub rotated: bool,
    pub x: Option<i32>,
    pub y: Option<i32>,
}


#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Tile {
    pub owner: Player,
    pub tile_type: TileSelector,
    pub id: String,
    pub rotated: bool,
    pub x: i32,
    pub y: i32,
    pub activated: bool,
    pub activation_resources: Vec<Resource>,
    pub exhausted: bool,
    pub additional_data: HashMap<String, String>,
    pub dmg_delt: i32,
    pub dmg_recieved: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct TileRecepie {
    pub cost: Vec<Resource>,
    pub footprint: Vec<Vec<bool>>,
    pub required_spaced_placement: bool,
    pub required_road_connection: bool,
    pub stats: TileStats,
    pub activated_costs: Vec<Vec<Resource>>,
}

impl From<NewTile> for Tile {
    fn from(ns: NewTile) -> Self {
        Self {
            owner: ns.owner,
            tile_type: ns.tile_type,
            id: ns.id,
            rotated: ns.rotated,
            x: ns.x.unwrap(),
            y: ns.y.unwrap(),
            activated: false,
            activation_resources: vec![],
            exhausted: false,
            additional_data: HashMap::new(),
            dmg_delt: 0,
            dmg_recieved: 0,
        }
    }
}