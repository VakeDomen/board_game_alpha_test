
use std::mem;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    game::game_models::{types::{tile_traits::{Tile, NewTile}, structure::{NewStructure, StructureSelector}, map::MapError}, functions::{upgrades::get_upgraders, ability_passive::get_passive_abilities, ability_active::{get_active_abilities, remove_resources}, placers::get_placers}, data::structures::{costs::get_costs, activation_costs::get_activation_costs}}, main};

use super::{lobby::new_game::NewGame, types::moves::{BugMove, Move, TechMove, TechMainPhaseMove, BugMainPhaseMove}, game_state::GameState};


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
    let mut que = mem::take(&mut state.move_que);
    for potential_move in &mut que {
        if state.player_turn == Player::First {
            if let Move::Tech(tech_move) = potential_move {
                if let TechMove::MainMove(main_move) = tech_move {
                    
                    // build structures
                    if let TechMainPhaseMove::Build(selector, x, y) = main_move {
                        if let Err(value) = build_structure(selector, state, x, y) {
                            return Err(value);
                        }
                    }

                    // activate buildings
                    if let TechMainPhaseMove::ActivateAbility(id, ability_index) = main_move {
                        if let Err(value) = activate_ability(state, id, ability_index) {
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
                    
                    if let BugMainPhaseMove::ActivateAbility(id, ability_index) = main_move {
                        if let Err(value) = activate_ability(state, id, ability_index) {
                            return Err(value);
                        }
                    }

                    if let BugMainPhaseMove::PlaceUnit(unit_selector, x, y, rotation) = main_move {
                       
                    }
                }
            }
        }
    }  

    state.move_que = que;
    state.turn_phase = TurnPhase::End;
    Ok(None)
}

fn build_structure(
    selector: &mut StructureSelector, 
    state: &mut GameState, 
    x: &mut i32, 
    y: &mut i32, 
) -> Result<(), ProgressionError> {
    let placers = get_placers();
    let costs = get_costs();
    if let Some(placer) = placers.get(&selector) {
        if let Some(placer) = placer {
            // create structure
            let id = Uuid::new_v4().to_string();
            let structure = NewStructure {
                structure_type: selector.clone(),
                id: id.clone(),
                x: None,
                y: None,
            };
            // place it
            match placer.place(NewTile::Structure(structure), state, *x, *y) {
                Ok(s) => state.tiles.insert(id, s),
                Err(e) => return Err(ProgressionError::CantPlaceBase(e)),
            };
            // subtract resources
            remove_resources(
                &mut state.tech_resources,
                &costs.get(&selector).unwrap()
            );
        }
    }
    Ok(())
}

fn activate_ability(
    state: &mut GameState, 
    id: &mut String, 
    ability_index: &mut i32, 
) -> Result<(), ProgressionError> {
    let ability_costs = get_activation_costs();
    let actives = get_active_abilities();
    let tile = match state.tiles.get(id) {
        Some(s) => s.clone(),
        None => return Err(ProgressionError::CantFindStructure(id.to_string())),
    };
    let mut structure = match tile {
        Tile::Structure(s) => s,
        Tile::Unit(_) => return Err(ProgressionError::CantActivateAbility(id.to_string())),
    };
    let cost = ability_costs.get(&structure.structure_type).unwrap();
    if cost.is_empty() || cost.len() <= *ability_index as usize {
        return Err(ProgressionError::CantActivateAbility(format!("no ability for {:#?}", structure.structure_type)));
    }
    if let Some(active) = actives.get(&structure.structure_type) {
        if let Some(active) = active {
            active.activate(state, &mut structure, cost[*ability_index as usize].clone());
        }
    }
    state.tiles.insert(id.to_string(), Tile::Structure(structure));
    Ok(())
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
                            let id = Uuid::new_v4().to_string();
                            let structure = NewStructure {
                                structure_type: StructureSelector::TechBase,
                                id: id.clone(),
                                x: None,
                                y: None,
                            };
                            match placer.place(NewTile::Structure(structure), state, *x, *y) {
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
                    if let Some(placer) = placers.get(&StructureSelector::BugBase1) {
                        if let Some(placer) = placer {
                            let id = Uuid::new_v4().to_string();
                            let structure = NewStructure {
                                structure_type: StructureSelector::BugBase1,
                                id: id.clone(),
                                x: None,
                                y: None,
                            };
                            match placer.place(NewTile::Structure(structure), state, *x, *y) {
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
    CantFindStructure(String),
    CantActivateAbility(String),
}