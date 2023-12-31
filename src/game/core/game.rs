
use crate::game::game_models::types::{structure::Structure, resource::Resouce};

use super::lobby::new_game::NewGame;


#[derive(Debug, Clone)]
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


#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    First,
    Second,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Tech(TechMove),
    Bug(BugMove),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TechMove {
    SetupBase(i32, i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BugMove {
    SetupBase(i32, i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TurnPhase {
    Setup,
    Dmg,
    Triggers,
    Main,
    End,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub player_turn: Player,
    pub winner: Option<Player>,
    pub turn_phase: TurnPhase,
    pub turn: i32,
    pub tiles: Vec<Tile>,
    pub move_que: Vec<Move>,
    pub tech_resources: Vec<Resouce>,
    pub bug_resources: Vec<Resouce>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Structure(Structure),
    Unit,
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_turn: Player::First, 
            turn: 0, 
            winner: None,
            tiles: vec![], 
            move_que: vec![], 
            turn_phase: TurnPhase::Setup,
            tech_resources: vec![],
            bug_resources: vec![Resouce::Nest],
            
        }
    }
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
                if let TechMove::SetupBase(x, y) = m {
                    setup_move = true;
                }
            }
        }
    }

    if state.player_turn == Player::Second {
        for player_move in state.move_que.iter() {
            if let Move::Bug(m) = player_move {
                if let BugMove::SetupBase(x, y) = m {
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
    NoBasePlacement
}