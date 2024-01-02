use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Resouce {
    Gold,
    Metal,
    Egg,
    Corpse,
    GiantEgg,
}