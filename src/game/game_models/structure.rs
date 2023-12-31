
use serde::Serialize;

use crate::game::core::game::GameState;

use super::{resource::Resouce, tiles::{Placable, Upgradable}};

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash)]
pub enum StructureSelector {
    BugBase,
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
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct StructureStats {
    pub hp: i32,
    pub attack: i32,
    pub range: i32,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Structure {
    pub structure_type: StructureSelector,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub stats: StructureStats,
    pub activated: bool,
    pub activation_resources: Vec<Resouce>,
    pub exhausted: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct StructureRecepie {
    pub cost: Vec<Resouce>,
    pub footprint: Vec<Vec<bool>>,
    pub stats: StructureStats,
    pub activated_costs: Vec<Vec<Resouce>>,
}


impl Placable for StructureRecepie {
    fn place(self, game_state: GameState) -> Option<Structure> {
        todo!()
    }

    fn can_place_on(self, game_state: GameState, x: i32, y: i32) -> bool {
        todo!()
    }

    fn has_enough_resources(self, game_state: GameState) -> bool {
        todo!()
    }
}