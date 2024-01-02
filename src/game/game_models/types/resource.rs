use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Resouce {
    Gold,
    Metal,
    Nest,
    Egg,
    Soul
}