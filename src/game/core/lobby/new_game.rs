use serde::Serialize;


#[derive(Debug, Serialize, Clone)]
pub struct NewGame {
    pub name: String,
    pub player1: String,
    pub player2: Option<String>
}