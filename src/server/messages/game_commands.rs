use serde::Serialize;

use crate::game::game_models::types::structure::StructureSelector;



#[derive(Debug, Serialize)]
pub enum GameCommand {
    BaseSetup(i32, i32),
    PlaceStructure(StructureSelector, i32, i32)
}

impl From<String> for GameCommand {
    fn from(value: String) -> Self {
        todo!()
    }
}