
use std::mem;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::game::game_models::{types::{tile_traits::{Tile, NewTile}, structure::{NewStructure, StructureSelector}, map::MapError}, functions::{upgrades::get_upgraders, ability_passive::get_passive_abilities, ability_active::get_active_abilities, placers::get_placers}};

use super::{lobby::new_game::NewGame, types::moves::{BugMove, Move, TechMove}, game_state::GameState};


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

fn progress_from_end(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
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

fn progress_from_main(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    state.turn_phase = TurnPhase::End;
    Ok(None)
}

fn progress_from_triggers(state: &mut GameState) -> Result<Option<GameState>, ProgressionError> {
    let upgraders = get_upgraders();
    let passives = get_passive_abilities();
    let actives = get_active_abilities();

    let bug_selectors = [
        StructureSelector::BugBase1,
        StructureSelector::BugBase2,
        StructureSelector::BugBase3,
    ];



    // trigger passives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if 
                (state.player_turn == Player::First && !bug_selectors.contains(&structure.structure_type)) ||
                (state.player_turn == Player::Second && bug_selectors.contains(&structure.structure_type))
            {
                if let Some(passive) = passives.get(&structure.structure_type) {
                    if let Some(passive) = passive {
                        passive.activate_passive(state, structure);
                    }
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger actives
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if 
                (state.player_turn == Player::First && !bug_selectors.contains(&structure.structure_type)) ||
                (state.player_turn == Player::Second && bug_selectors.contains(&structure.structure_type))
            {
                if let Some(active) = actives.get(&structure.structure_type) {
                    if let Some(active) = active {
                        active.trigger(state, structure);
                    }
                }
            }
        }
    }  
    state.tiles = tiles; // Put the original tiles back.

    // trigger upgrade structs
    let mut tiles = mem::take(&mut state.tiles); // Temporarily take ownership of the tiles.
    for (_, tile) in &mut tiles {
        if let Tile::Structure(structure) = tile {
            if 
                (state.player_turn == Player::First && !bug_selectors.contains(&structure.structure_type)) ||
                (state.player_turn == Player::Second && bug_selectors.contains(&structure.structure_type))
            {
                if let Some(upgrader) = upgraders.get(&structure.structure_type) {
                    if let Some(upgrader) = upgrader {
                        upgrader.upgrade(state, structure); // Now it's okay to borrow state mutably.
                    }
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

    let placers = get_placers();

    let mut setup_move = false;

    // trigger upgrade structs
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let TechMove::SetupMove(x, y) = tech_move {
                    if let Some(placer) = placers.get(&StructureSelector::TechBase) {
                        if let Some(placer) = placer {
                            let structure = NewStructure {
                                structure_type: StructureSelector::TechBase,
                                id: Uuid::new_v4().to_string(),
                                x: None,
                                y: None,
                            };
                            if let Ok(_) = placer.place(NewTile::Structure(structure), state, *x, *y) {
                                setup_move = true;
                            }
                        }
                    }
                }
            }
            continue;
        }

        if state.player_turn == Player::Second {
            if let Move::Bug(bug_move) = potential_move {
                if let BugMove::SetupMove(x, y) = bug_move {
                    if let Some(placer) = placers.get(&StructureSelector::BugBase1) {
                        if let Some(placer) = placer {
                            let structure = NewStructure {
                                structure_type: StructureSelector::BugBase1,
                                id: Uuid::new_v4().to_string(),
                                x: None,
                                y: None,
                            };
                            if let Ok(_) = placer.place(NewTile::Structure(structure), state, *x, *y) {
                                setup_move = true;
                            }
                        }
                    }
                }
            }
        }
    }  

    state.move_que = que;
    // if state.player_turn == Player::First {
    //     for player_move in state.move_que.iter() {
    //         let mut executed = false;
    //         if let Move::Tech(m) = player_move {
    //             if let TechMove::SetupMove(x, y) = m {
    //                 let structure = NewStructure {
    //                     structure_type: StructureSelector::TechBase,
    //                     id: Uuid::new_v4().to_string(),
    //                     x: None,
    //                     y: None,
    //                 };
    //                 match state.map.place_structure(structure, state.tiles.clone(), *x, *y) {
    //                     Ok(s) => state.tiles.insert(s.id.clone(), Tile::Structure(s)),
    //                     Err(e) => return Err(ProgressionError::CantPlaceBase(e)),
    //                 };
    //                 setup_move = true;
    //                 executed = true;
    //             }
    //             if executed {
    //                 state.executed_moves.push(player_move.clone());
    //             }
    //         }
    //     }
    // }

    // if state.player_turn == Player::Second {
    //     for player_move in state.move_que.iter() {
    //         let mut executed = false;
    //         if let Move::Bug(m) = player_move {
    //             if let BugMove::SetupMove(x, y) = m {
    //                 let structure = NewStructure {
    //                     structure_type: StructureSelector::BugBase1,
    //                     id: Uuid::new_v4().to_string(),
    //                     x: None,
    //                     y: None,
    //                 };
    //                 match state.map.place_structure(structure, state.tiles.clone(), *x, *y) {
    //                     Ok(s) => state.tiles.insert(s.id.clone(), Tile::Structure(s)),
    //                     Err(e) => return Err(ProgressionError::CantPlaceBase(e)),
    //                 };
    //                 setup_move = true;
    //                 executed = true;
    //             }
    //         }
    //         if executed {
    //             state.executed_moves.push(player_move.clone());
    //         }
    //     }
    // }

    if setup_move {
        if state.player_turn == Player::First {
            state.player_turn = Player::Second;
        } else {
            state.player_turn = Player::First;
            state.turn_phase = TurnPhase::Dmg;
        }
        state.move_que = vec![];
        Ok(None)
    } else {
        Err(ProgressionError::NoBasePlacement)
    }
}

#[derive(Debug)]
pub enum ProgressionError {
    NoBasePlacement,
    CantPlaceBase(MapError),
}