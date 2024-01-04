
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game::game_models::data::structures::stats::StructureStats;

use super::resource::Resource;


#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum StructureSelector {
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
}

#[derive(Debug, Clone)]
pub struct NewStructure {
    pub structure_type: StructureSelector,
    pub id: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
}


#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Structure {
    pub structure_type: StructureSelector,
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub activated: bool,
    pub activation_resources: Vec<Resource>,
    pub exhausted: bool,
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StructureRecepie {
    pub cost: Vec<Resource>,
    pub footprint: Vec<Vec<bool>>,
    pub required_spaced_placement: bool,
    pub required_road_connection: bool,
    pub stats: StructureStats,
    pub activated_costs: Vec<Vec<Resource>>,
}

impl From<NewStructure> for Structure {
    fn from(ns: NewStructure) -> Self {
        Self {
            structure_type: ns.structure_type,
            id: ns.id,
            x: ns.x.unwrap(),
            y: ns.y.unwrap(),
            activated: false,
            activation_resources: vec![],
            exhausted: false,
            additional_data: HashMap::new(),
        }
    }
}