use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Eq, Clone, Hash, PartialEq)]
pub enum UnitSelector {
    BugSoldierLV1,
    BugSoldierLV2,
    BugSoldierLV3,
    BugEliteMelee,
    BugEliteRanged,
}


#[derive(Debug)]
pub struct NewUnit {
    pub unit_type: UnitSelector,
    pub id: String,
    pub rotated: bool,
    pub x: Option<i32>,
    pub y: Option<i32>,
}


#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Unit {
    pub unit_type: UnitSelector,
    pub id: String,
    pub rotated: bool,
    pub x: i32,
    pub y: i32,
    pub exhausted: bool,
}

impl From<NewUnit> for Unit {
    fn from(nu: NewUnit) -> Self {
        Self {
            unit_type: nu.unit_type,
            id: nu.id,
            rotated: nu.rotated,
            x: nu.x.unwrap(),
            y: nu.y.unwrap(),
            exhausted: false,
        }
    }
}