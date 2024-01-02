
use std::collections::HashMap;

use serde::Serialize;

use crate::game::game_models::data::stats::StructureStats;

use super::resource::Resouce;


#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash)]
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



#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Structure {
    pub structure_type: StructureSelector,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub activated: bool,
    pub activation_resources: Vec<Resouce>,
    pub exhausted: bool,
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StructureRecepie {
    pub cost: Vec<Resouce>,
    pub footprint: Vec<Vec<bool>>,
    pub spaced_placement: bool,
    pub stats: StructureStats,
    pub activated_costs: Vec<Vec<Resouce>>,
}