use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Resource {
    Gold,
    Metal,
    Egg,
    Corpse,
    GiantEgg,
}