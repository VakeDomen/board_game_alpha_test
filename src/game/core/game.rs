
use std::{mem, collections::HashMap};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::game::game_models::{
    types::{
        map::{MapError, Interactor, TileOption}, tile::{TileSelector, NewTile}, 
    }, 
    functions::{
        upgrades::get_upgraders, 
        ability_passive::get_passive_abilities, 
        ability_active::get_active_abilities
    }, 
    data::{
        activation_costs::get_activation_costs, 
        placers::get_placers, stats::get_stats
    },
};
use super::{
    lobby::new_game::NewGame, 
    game_state::GameState, types::moves::{Move, PhaseMove, MainPhaseMove}
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub player1: String,
    pub player2: String,
    pub states: Vec<GameState>,
}

impl From<NewGame> for Game {
    fn from(ng: NewGame) -> Self {
        let player2 = ng.player2.unwrap();
        Game {
            name: ng.name,
            player1: ng.player1,
            player2,
            states: vec![GameState::default()],
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Player {
    First,
    Second,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TurnPhase {
    Setup,
    Dmg,
    Triggers,
    Main,
    End,
}

impl Game {

    pub fn apply_state(&mut self) -> Result<(), ProgressionError> { 
        let mut current_state = self.states.last_mut().unwrap();
        match current_state.turn_phase {
            TurnPhase::Setup => apply_setup(&mut current_state),
            TurnPhase::Dmg => apply_dmg(&mut current_state),
            TurnPhase::Triggers => apply_triggers(&mut current_state),
            TurnPhase::Main => apply_main(&mut current_state),
            TurnPhase::End => apply_end(&mut current_state),
        }
    }

    pub fn progress_state(&mut self) -> Result<(), ProgressionError> {
        let mut current_state = self.states.last_mut().unwrap();
        let result = match current_state.turn_phase {
            TurnPhase::Setup => progress_from_setup(&mut current_state),
            TurnPhase::Dmg => progress_from_dmg(&mut current_state),
            TurnPhase::Triggers => progress_from_triggers(&mut current_state),
            TurnPhase::Main => progress_from_main(&mut current_state),
            TurnPhase::End => progress_from_end(&mut current_state),
        };
        let potenital_next_state = match result {
            Ok(next_state_option) => next_state_option,
            Err(e) => {
                println!("Error progressing to next state: {:#?}", e);
                return Err(e);
            },
        };
        if let Some(state) = potenital_next_state {
            self.states.push(state);
        }
        Ok(())
    }
}

fn progress_from_setup(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    apply_setup(state)?;

    let mut found_base = false;

    for tile in state.tiles.values() {
        if state.player_turn == Player::First {
            if tile.tile_type == TileSelector::TechBase {
                found_base = true;
            }
        }
        if state.player_turn == Player::Second {
            if tile.tile_type == TileSelector::BugBase1 {
                found_base = true;
            }
        }
    }

    if !found_base {
        return Err(ProgressionError::NoBasePlacement);
    }

    if state.player_turn == Player::First {
        state.player_turn = Player::Second;
    } else {
        state.player_turn = Player::First;
        state.turn_phase = TurnPhase::Dmg;
    }
    state.move_que = vec![];
    Ok(None)
}

fn progress_from_dmg(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    apply_dmg(state)?;
    state.turn_phase = TurnPhase::Triggers;
    Ok(None)
}

fn progress_from_triggers(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    apply_triggers(state)?;
    state.turn_phase = TurnPhase::Main;
    Ok(None)
}

fn progress_from_main(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    apply_main(state)?;
    state.turn_phase = TurnPhase::End;
    Ok(None)
}

fn progress_from_end(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    apply_end(state)?;
    let mut new_state = state.clone();
    // next player turn in the next state
    new_state.turn_phase = TurnPhase::Dmg;
    new_state.move_que = vec![];
    new_state.executed_moves = vec![];
    new_state.turn += 1;

    if state.player_turn == Player::First {
        new_state.player_turn = Player::Second;
    } else {
        new_state.player_turn = Player::First;
    }
    // check for game end
    Ok(Some(new_state))
}

fn apply_setup(state: &mut GameState) -> Result<(), ProgressionError> {
    // trigger upgrade structs
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let PhaseMove::SetupMove(x, y) = tech_move {
                    place_tile(&mut TileSelector::TechBase, &mut 0, state, x, y)?;   
                }
            }
            continue;
        }

        if state.player_turn == Player::Second {
            if let Move::Bug(bug_move) = potential_move {
                if let PhaseMove::SetupMove(x, y) = bug_move {
                    place_tile(&mut TileSelector::BugBase1, &mut 0, state, x, y)?; 
                }
            }
        }
    }  
    state.move_que = que;
    Ok(())
}

fn apply_dmg(state: &mut GameState) -> Result<(), ProgressionError> {
    let mut que = mem::take(&mut state.move_que);
    let stats =get_stats();
    for potential_move in &mut que {

        let phase_move = match potential_move {
            Move::Tech(pm) => pm,
            Move::Bug(pm) => pm,
        };

        if let PhaseMove::DmgMove(initiator_id, target_id, dmg) = phase_move {
            
            let initiator = state.tiles.get_mut(initiator_id);
            if let None = initiator {
                return Err(ProgressionError::NoDmgInitiatorFound);
            }
            let initiator = initiator.unwrap();
            
            if stats.get(&initiator.tile_type).unwrap().attack - initiator.dmg_delt < *dmg {
                return Err(ProgressionError::InitiatorDealingTooMuchDmg);
            }

            initiator.dmg_delt += *dmg;

            let target = state.tiles.get_mut(target_id);
            if let None = target {
                return Err(ProgressionError::NoDmgTargetFound);
            }
            let target = target.unwrap();

            target.dmg_recieved += *dmg;

            // if dead
            if stats.get(&target.tile_type).unwrap().hp - target.dmg_recieved <= 0 {
                // remove target
                for (loc, tile) in state.map.get_footprint_tiles_by_id(&target_id).iter() {
                    if let TileOption::Id(id) = tile {
                        state.tiles.remove(id);
                    }
                    state.map[loc.0][loc.1] = "".to_owned();
                }
            }
        }
    }  
    state.move_que = que;
    Ok(())
}


fn apply_triggers(state: &mut GameState) -> Result<(), ProgressionError> {
    let upgraders = get_upgraders();
    let passives = get_passive_abilities();
    let actives = get_active_abilities();

    // trigger passives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Some(passive) = passives.get(&tile.tile_type) {
            if let Some(passive) = passive {
                passive.activate_passive(state, tile);
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger actives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Some(active) = actives.get(&tile.tile_type) {
            if let Some(active) = active {
                active.trigger(state, tile);
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger upgrade structs
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Some(upgrader) = upgraders.get(&tile.tile_type) {
            if let Some(upgrader) = upgrader {
                upgrader.upgrade(state, tile); // Now it's okay to borrow state mutably.
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.


    // check if T1 nests need to be placed
    for row in 0..state.map.len() {
        for col in 0..state.map[row].len() {
            if !state.map[row][col].is_empty() {
                continue;
            }

            let adj = state.map.get_tile_adjacent(row as i32, col as i32);
            let mut all_t1_tiles = true;
            for (_, tile_option) in adj.iter() {
                let tile = match tile_option {
                    TileOption::Id(id) => state.tiles.get(id).unwrap(),
                    TileOption::None => {
                        all_t1_tiles = false;
                        break; 
                    },
                    TileOption::OutOfBounds =>{
                        all_t1_tiles = false;
                        break; 
                    },
                };
                if TileSelector::BugSoldierLV1 != tile.tile_type {
                    all_t1_tiles = false;
                    break; 
                }
            }
            if !all_t1_tiles {
                continue;
            }
            for (loc, tile) in adj.iter() {
                if let TileOption::Id(id) = tile {
                    state.tiles.remove(id);
                }
                state.map[loc.0][loc.1] = "".to_owned();
            }

            place_tile(
                &mut TileSelector::BugBase1, 
                &mut 0, 
                state, 
                &mut (row as i32), 
                &mut (col as i32)
            )?;            
        }
    }
    Ok(())
}

fn apply_main(state: &mut GameState) -> Result<(), ProgressionError> {
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let PhaseMove::MainMove(main_move) = tech_move {
                    
                    // build tiles
                    if let MainPhaseMove::PlaceTile(selector, x, y, rotation) = main_move {
                        if let Err(value) = place_tile(selector, rotation, state, x, y) {
                            return Err(value);
                        }
                    }

                    // activate tiles
                    if let MainPhaseMove::ActivateAbility(id, ability_index, additional_data) = main_move {
                        if let Err(value) = activate_ability(state, id, ability_index, additional_data)  {
                            return Err(value);
                        }
                    }
                }
            }
            continue;
        }

        if state.player_turn == Player::Second {
            if let Move::Bug(bug_move) = potential_move {
                if let PhaseMove::MainMove(main_move) = bug_move {
                    
                    // build tiles
                    if let MainPhaseMove::PlaceTile(tile_selector, x, y, rotation) = main_move {
                        if let Err(value) = place_tile(tile_selector, rotation, state, x, y) {
                            return Err(value);
                        }
                    }

                    // activate tiles
                    if let MainPhaseMove::ActivateAbility(id, ability_index, additional_data) = main_move {
                        if let Err(value) = activate_ability(state, id, ability_index, additional_data) {
                            return Err(value);
                        }
                    }

                }
            }
        }
    }  
    state.move_que = que;
    Ok(())
}

fn apply_end(_: &mut GameState) -> Result<(), ProgressionError> {
    Ok(())
}



fn place_tile(
    tile_selector: &mut TileSelector, 
    rotation: &mut i32, 
    state: &mut GameState, 
    x: &mut i32, 
    y: &mut i32
) -> Result<(), ProgressionError> {
    let placers = get_placers();
    if let Some(placer) = placers.get(&tile_selector) {
        if let Some(placer) = placer {
            // create tile
            let id = Uuid::new_v4().to_string();
            let tile = NewTile {
                owner: state.player_turn.clone(),
                tile_type: tile_selector.clone(),
                id: id.clone(),
                x: None,
                y: None,
                rotated: (*rotation == 1),
            };
            // place it
            match placer.place(tile, state, *x, *y) {
                Ok(s) => state.tiles.insert(id, s),
                Err(e) => return Err(ProgressionError::CantPlaceTile(e)),
            };
        }
    }
    Ok(())
}

fn activate_ability(
    state: &mut GameState, 
    id: &mut String, 
    ability_index: &mut i32, 
    additional_data: &mut HashMap<String, String>,
) -> Result<(), ProgressionError> {
    let ability_costs = get_activation_costs();
    let actives = get_active_abilities();
    let mut tile = match state.tiles.get(id) {
        Some(s) => s.clone(),
        None => return Err(ProgressionError::CantFindTile(id.to_string())),
    };
    
    tile.additional_data = additional_data.clone();
    let cost = ability_costs.get(&tile.tile_type).unwrap();
    if cost.is_empty() || cost.len() <= *ability_index as usize {
        return Err(ProgressionError::CantActivateAbility(format!("no ability for {:#?}", tile.tile_type)));
    }
    if let Some(active) = actives.get(&tile.tile_type) {
        if let Some(active) = active {
            active.activate(state, &mut tile, cost[*ability_index as usize].clone());
        }
    }
    state.tiles.insert(id.to_string(), tile);
    Ok(())
}





#[derive(Debug)]
pub enum ProgressionError {
    NoBasePlacement,
    CantFindTile(String),
    CantActivateAbility(String),
    CantPlaceTile(MapError),
    NoDmgInitiatorFound,
    NoDmgTargetFound,
    InitiatorDealingTooMuchDmg,
}