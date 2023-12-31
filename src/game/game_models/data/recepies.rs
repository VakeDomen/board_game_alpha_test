use std::collections::HashMap;

use crate::game::game_models::types::structure::{StructureSelector, StructureRecepie};

use super::{stats::get_stats, activation_costs::get_activation_costs, footprints::get_footprints, costs::get_costs};




pub fn get_recepies() -> HashMap<StructureSelector, StructureRecepie> {
    let mut recepies: HashMap<StructureSelector, StructureRecepie> = HashMap::new();

    recepies.insert(StructureSelector::BugBase1, construct_recepie(StructureSelector::BugBase1));
    recepies.insert(StructureSelector::BugBase2, construct_recepie(StructureSelector::BugBase2));
    recepies.insert(StructureSelector::BugBase3, construct_recepie(StructureSelector::BugBase3));
    recepies.insert(StructureSelector::TechBase, construct_recepie(StructureSelector::TechBase));
    recepies.insert(StructureSelector::TechRoad, construct_recepie(StructureSelector::TechRoad));
    recepies.insert(StructureSelector::TechMine1, construct_recepie(StructureSelector::TechMine1));
    recepies.insert(StructureSelector::TechMine2, construct_recepie(StructureSelector::TechMine2));
    recepies.insert(StructureSelector::TechRefinery1, construct_recepie(StructureSelector::TechRefinery1));
    recepies.insert(StructureSelector::TechRefinery2, construct_recepie(StructureSelector::TechRefinery2));
    recepies.insert(StructureSelector::TechMarket, construct_recepie(StructureSelector::TechMarket));
    recepies.insert(StructureSelector::TechTurret1, construct_recepie(StructureSelector::TechTurret1));
    recepies.insert(StructureSelector::TechTurret2, construct_recepie(StructureSelector::TechTurret2));
    recepies.insert(StructureSelector::TechArtillery1, construct_recepie(StructureSelector::TechArtillery1));
    recepies.insert(StructureSelector::TechArtillery2, construct_recepie(StructureSelector::TechArtillery2));
    recepies.insert(StructureSelector::TechWall1, construct_recepie(StructureSelector::TechWall1));
    
    recepies
}


pub fn construct_recepie(selector: StructureSelector) -> StructureRecepie {
    let stats = get_stats();
    let activation_costs = get_activation_costs();
    let footprints = get_footprints();
    let costs = get_costs();
    StructureRecepie {
        cost: costs.get(&selector).unwrap().clone(),
        footprint: footprints.get(&selector).unwrap().clone(),
        stats: stats.get(&selector).unwrap().clone(),
        activated_costs: activation_costs.get(&selector).unwrap().clone(),
    }
}