use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Resouce {
    Gold,
    Metal,
    Nest,
    Egg,
    Soul
}