use std::{collections::HashMap, vec};

use crate::game::game_models::types::{structure::StructureSelector, resource::Resource};


pub fn get_activation_costs() -> HashMap<StructureSelector, Vec<Vec<Resource>>> {
    let mut hm = HashMap::new();
    hm.insert(StructureSelector::BugBase1, vec![],);
    hm.insert(StructureSelector::BugBase2, vec![],);
    hm.insert(StructureSelector::BugBase3, vec![
        vec![
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
            Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, Resource::Corpse, 
        ]
    ],);
    hm.insert(StructureSelector::TechBase, vec![]);
    hm.insert(StructureSelector::TechRoad, vec![]);
    hm.insert(StructureSelector::TechMine1, vec![
        vec![Resource::Metal]
    ]);
    hm.insert(StructureSelector::TechMine2, vec![]);
    hm.insert(StructureSelector::TechRefinery1, vec![
        vec![Resource::Metal],
        vec![Resource::Gold]
    ]);
    hm.insert(StructureSelector::TechRefinery2, vec![
        vec![Resource::Gold]
    ]);
    hm.insert(StructureSelector::TechMarket, vec![
        vec![Resource::Gold],
        vec![Resource::Metal]
    ]);
    hm.insert(StructureSelector::TechTurret1, vec![
        vec![Resource::Metal],
    ]);
    hm.insert(StructureSelector::TechTurret2, vec![]);
    hm.insert(StructureSelector::TechArtillery1, vec![
        vec![Resource::Metal],
    ]);
    hm.insert(StructureSelector::TechArtillery2, vec![]);
    hm.insert(StructureSelector::TechWall1, vec![]);
    hm.insert(StructureSelector::TechNuke, vec![
        vec![Resource::Metal, Resource::Metal, Resource::Metal],
    ]);

    hm
}