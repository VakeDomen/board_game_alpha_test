use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Resouce {
    Gold,
    Metal,
    Egg,
    Soul
}