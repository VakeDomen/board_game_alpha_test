
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
        placers::get_placers
    },
};
use super::{
    lobby::new_game::NewGame, 
    types::moves::{BugMove, Move, TechMove, TechMainPhaseMove, BugMainPhaseMove}, 
    game_state::GameState
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
    if state.move_que.is_empty() {
        return Err(ProgressionError::NoBasePlacement);
    }

    let placers = get_placers();

    let mut setup_move = false;

    // trigger upgrade structs
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let TechMove::SetupMove(x, y) = tech_move {
                    if let Some(placer) = placers.get(&TileSelector::TechBase) {
                        if let Some(placer) = placer {
                            let id = Uuid::new_v4().to_string();
                            let tile = NewTile {
                                tile_type: TileSelector::TechBase,
                                id: id.clone(),
                                x: None,
                                y: None,
                                rotated: false,
                            };
                            match placer.place(tile, state, *x, *y) {
                                Ok(s) => state.tiles.insert(id, s),
                                Err(e) => return Err(ProgressionError::CantPlaceBase(e)),
                            };
                            setup_move = true;
                        }
                    }
                }
            }
            continue;
        }

        if state.player_turn == Player::Second {
            if let Move::Bug(bug_move) = potential_move {
                if let BugMove::SetupMove(x, y) = bug_move {
                    if let Some(placer) = placers.get(&TileSelector::BugBase1) {
                        if let Some(placer) = placer {
                            let id = Uuid::new_v4().to_string();
                            let tile = NewTile {
                                tile_type: TileSelector::BugBase1,
                                id: id.clone(),
                                x: None,
                                y: None,
                                rotated: false,
                            };
                            match placer.place(tile, state, *x, *y) {
                                Ok(s) => state.tiles.insert(id, s),
                                Err(e) => return Err(ProgressionError::CantPlaceBase(e)),
                            };
                            setup_move = true;
                        }
                    }
                }
            }
        }
    }  

    state.move_que = que;

    if !setup_move {
        return Err(ProgressionError::NoBasePlacement);
    }

    Ok(())
}

fn apply_dmg(state: &mut GameState) -> Result<(), ProgressionError> {
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let TechMove::DmgMove(initiator_id, target_id, dmg) = tech_move {
                    todo!()
                }
            }
            continue;
        }

        if state.player_turn == Player::Second {
            if let Move::Bug(bug_move) = potential_move {
                if let BugMove::DmgMove(initiator_id, target_id, dmg) = bug_move {
                    
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

    let bug_selectors = [
        TileSelector::BugBase1,
        TileSelector::BugBase2,
        TileSelector::BugBase3,
    ];



    // trigger passives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if 
            (state.player_turn == Player::First && !bug_selectors.contains(&tile.tile_type)) ||
            (state.player_turn == Player::Second && bug_selectors.contains(&tile.tile_type))
        {
            if let Some(passive) = passives.get(&tile.tile_type) {
                if let Some(passive) = passive {
                    passive.activate_passive(state, tile);
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger actives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if 
            (state.player_turn == Player::First && !bug_selectors.contains(&tile.tile_type)) ||
            (state.player_turn == Player::Second && bug_selectors.contains(&tile.tile_type))
        {
            if let Some(active) = actives.get(&tile.tile_type) {
                if let Some(active) = active {
                    active.trigger(state, tile);
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger upgrade structs
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if 
            (state.player_turn == Player::First && !bug_selectors.contains(&tile.tile_type)) ||
            (state.player_turn == Player::Second && bug_selectors.contains(&tile.tile_type))
        {
            if let Some(upgrader) = upgraders.get(&tile.tile_type) {
                if let Some(upgrader) = upgrader {
                    upgrader.upgrade(state, tile); // Now it's okay to borrow state mutably.
                }
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
            for (loc, _) in adj.iter() {
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
                if let TechMove::MainMove(main_move) = tech_move {
                    
                    // build tiles
                    if let TechMainPhaseMove::Build(selector, x, y) = main_move {
                        if let Err(value) = place_tile(selector, &mut 0, state, x, y) {
                            return Err(value);
                        }
                    }

                    // activate buildings
                    if let TechMainPhaseMove::ActivateAbility(id, ability_index, additional_data) = main_move {
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
                if let BugMove::MainMove(main_move) = bug_move {
                    
                    if let BugMainPhaseMove::ActivateAbility(id, ability_index, additional_data) = main_move {
                        if let Err(value) = activate_ability(state, id, ability_index, additional_data) {
                            return Err(value);
                        }
                    }

                    if let BugMainPhaseMove::PlaceTile(tile_selector, x, y, rotation) = main_move {
                        if let Err(value) = place_tile(tile_selector, rotation, state, x, y) {
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
    CantPlaceBase(MapError),
    CantFindTile(String),
    CantActivateAbility(String),
    CantPlaceTile(MapError),
    NoDmgInitiatorFound,
    NoDmgTargetFound,
}