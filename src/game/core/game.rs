
use std::mem;

use serde::Serialize;
use uuid::Uuid;

use crate::game::game_models::{types::{tile_traits::Tile, structure::{NewStructure, StructureSelector}, map::StructurePlacer}, functions::{upgrades::get_upgraders, ability_passive::get_passive_abilities, ability_active::get_active_abilities}};

use super::{lobby::new_game::NewGame, types::moves::{BugMove, Move, TechMove}, game_state::GameState};


#[derive(Debug, Clone, Serialize)]
pub struct Game {
    pub player1: String,
    pub player2: String,
    pub states: Vec<GameState>,
}

impl From<NewGame> for Game {
    fn from(ng: NewGame) -> Self {
        let player2 = ng.player2.unwrap();
        Game {
            player1: ng.player1,
            player2,
            states: vec![GameState::default()],
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Player {
    First,
    Second,
}



#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TurnPhase {
    Setup,
    Dmg,
    Triggers,
    Main,
    End,
}


impl Game {
    fn progress_state(mut self) -> Result<(), ProgressionError> {
        let mut current_state = self.states.last_mut().unwrap();
        let result = match current_state.turn_phase {
            TurnPhase::Setup => progress_from_setup(&mut current_state),
            TurnPhase::Dmg => progress_from_dmg(&mut current_state),
            TurnPhase::Triggers => progress_from_triggers(&mut current_state),
            TurnPhase::Main => progress_from_main(&mut current_state),
            TurnPhase::End => progress_from_end(&mut current_state),
        };
        let potenital_next_state = match result {
            Ok(potenital_next_state) => potenital_next_state,
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

fn progress_from_end(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    let mut new_state = state.clone();

    // next player turn in the next state
    new_state.turn_phase = TurnPhase::Dmg;
    if state.player_turn == Player::First {
        new_state.player_turn = Player::Second;
    } else {
        new_state.player_turn = Player::First;
    }


    // check for game end


    Ok(Some(new_state))
}

fn progress_from_main(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    state.turn_phase = TurnPhase::End;
    Ok(None)
}

fn progress_from_triggers(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    let upgraders = get_upgraders();
    let passives = get_passive_abilities();
    let actives = get_active_abilities();

    // trigger passives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if let Some(passive) = passives.get(&structure.structure_type) {
                if let Some(passive) = passive {
                    passive.activate_passive(state, structure);
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger actives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if let Some(active) = actives.get(&structure.structure_type) {
                if let Some(active) = active {
                    active.trigger(state, structure);
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger upgrade structs
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if let Some(upgrader) = upgraders.get(&structure.structure_type) {
                if let Some(upgrader) = upgrader {
                    upgrader.upgrade(state, structure); // Now it's okay to borrow state mutably.
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.
    
    
    state.turn_phase = TurnPhase::Main;
    Ok(None)
}

fn progress_from_dmg(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    state.turn_phase = TurnPhase::Triggers;
    Ok(None)
}

fn progress_from_setup(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    if state.move_que.is_empty() {
        return Err(ProgressionError::NoBasePlacement);
    }

    let mut setup_move = false;

    if state.player_turn == Player::First {
        for player_move in state.move_que.iter() {
            if let Move::Tech(m) = player_move {
                if let TechMove::SetupMove(x, y) = m {
                    let structure = NewStructure {
                        structure_type: StructureSelector::TechBase,
                        id: Uuid::new_v4().to_string(),
                        x: None,
                        y: None,
                    };
                    if let Ok(s) = state.map.place_structure(structure, state.tiles.clone(), *x, *y) {
                        state.tiles.insert(s.id.clone(), Tile::Structure(s));
                    } else {
                        return Err(ProgressionError::CantPlaceBase);
                    }
                    setup_move = true;
                }
            }
        }
    }

    if state.player_turn == Player::Second {
        for player_move in state.move_que.iter() {
            if let Move::Bug(m) = player_move {
                if let BugMove::SetupMove(x, y) = m {
                    let structure = NewStructure {
                        structure_type: StructureSelector::BugBase1,
                        id: Uuid::new_v4().to_string(),
                        x: None,
                        y: None,
                    };
                    if let Ok(s) = state.map.place_structure(structure, state.tiles.clone(), *x, *y) {
                        state.tiles.insert(s.id.clone(), Tile::Structure(s));
                    } else {
                        return Err(ProgressionError::CantPlaceBase);
                    }
                    setup_move = true;
                }
            }
        }
    }

    if setup_move {
        state.turn_phase = TurnPhase::Dmg;
        Ok(None)
    } else {
        Err(ProgressionError::NoBasePlacement)
    }
}

#[derive(Debug)]
pub enum ProgressionError {
    NoBasePlacement,
    CantPlaceBase,
}