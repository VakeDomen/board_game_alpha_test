use super::{new_game::NewGame, game_commands::GameCommand};


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
pub enum PlayerTurn {
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
    pub player_turn: PlayerTurn,
    pub finalized: bool,
    pub turn: i32,
    pub tiles: Vec<Tile>,
    pub move_que: Vec<Move>,
    pub turn_phase: TurnPhase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    TechBase,
    Road,
    BugBase1,
    BugBase2,
    BugBase3,
    BugSoldier1,
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_turn: PlayerTurn::First, 
            finalized: false, 
            turn: 0, 
            tiles: vec![], 
            move_que: vec![], 
            turn_phase: TurnPhase::Setup,
        }
    }
}

impl Game {
    fn progress_state(self) -> Result<(), ProgressionError> {
        let current_state = self.states.last().unwrap();
        match current_state.turn_phase {
            TurnPhase::Setup => progress_from_setup(current_state),
            TurnPhase::Dmg => progress_from_dmg(current_state),
            TurnPhase::Triggers => progress_from_triggers(current_state),
            TurnPhase::Main => progress_from_main(current_state),
            TurnPhase::End => progress_from_end(current_state),
        }          
    }
}

fn progress_from_end(state: &GameState) -> Result<(), ProgressionError> {
    todo!()
}

fn progress_from_main(state: &GameState) -> Result<(), ProgressionError> {
    todo!()
}

fn progress_from_triggers(state: &GameState) -> Result<(), ProgressionError> {
    todo!()
}

fn progress_from_dmg(state: &GameState) -> Result<(), ProgressionError> {
    todo!()
}

fn progress_from_setup(state: &GameState) -> Result<(), ProgressionError> {
    if state.move_que.is_empty() {
        return Err(ProgressionError::NoBasePlacement);
    }

    let mut setup_move = false;

    
    if state.player_turn == PlayerTurn::First {
        for player_move in state.move_que.iter() {
            if let Move::Tech(m) = player_move {
                if let TechMove::SetupBase(x, y) = m {
                    setup_move = true;
                }
            }
        }
    }

    if state.player_turn == PlayerTurn::Second {
        for player_move in state.move_que.iter() {
            if let Move::Bug(m) = player_move {
                if let BugMove::SetupBase(x, y) = m {
                    setup_move = true;
                }
            }
        }
    }

    if setup_move {
        Ok(())
    } else {
        Err(ProgressionError::NoBasePlacement)
    }
}

pub enum ProgressionError {
    NoBasePlacement
}