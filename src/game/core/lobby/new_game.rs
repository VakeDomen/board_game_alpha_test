use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct NewGame {
    pub name: String,
    pub player1: String,
    pub player2: Option<String>
}