use std::collections::HashMap;

use serde::{Serialize, Deserialize};


use crate::game::game_models::types::{tile::Tile, resource::Resource, map::Map};

use super::{game::{Player, TurnPhase}, types::moves::Move};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player_turn: Player,
    pub winner: Option<Player>,
    pub turn_phase: TurnPhase,
    pub turn: i32,
    pub tiles: HashMap<String, Tile>,
    pub move_que: Vec<Move>,
    pub executed_moves: Vec<Move>,
    pub tech_resources: Vec<Resource>,
    pub bug_resources: Vec<Resource>,
    pub map: Map,
}


impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_turn: Player::First, 
            turn: 0, 
            winner: None,
            tiles: HashMap::new(), 
            move_que: vec![], 
            executed_moves: vec![],
            turn_phase: TurnPhase::Setup,
            tech_resources: vec![],
            bug_resources: vec![],
            map: vec![vec!["".to_string(); 17]; 34],
        }
    }
}