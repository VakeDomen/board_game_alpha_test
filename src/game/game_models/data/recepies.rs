use std::collections::HashMap;


use crate::game::game_models::types::tile::{TileRecepie, TileSelector};

use super::{stats::get_stats, activation_costs::get_activation_costs, footprints::get_footprints, costs::get_costs, spaced_placements::get_spaced_placements, road_connection_requirements::get_road_connection_requirements};




pub fn get_recepies() -> HashMap<TileSelector, TileRecepie> {
    let mut recepies: HashMap<TileSelector, TileRecepie> = HashMap::new();

    recepies.insert(TileSelector::BugBase1, get_recepie(&TileSelector::BugBase1));
    recepies.insert(TileSelector::BugBase2, get_recepie(&TileSelector::BugBase2));
    recepies.insert(TileSelector::BugBase3, get_recepie(&TileSelector::BugBase3));
    recepies.insert(TileSelector::TechBase, get_recepie(&TileSelector::TechBase));
    recepies.insert(TileSelector::TechRoad, get_recepie(&TileSelector::TechRoad));
    recepies.insert(TileSelector::TechMine1, get_recepie(&TileSelector::TechMine1));
    recepies.insert(TileSelector::TechMine2, get_recepie(&TileSelector::TechMine2));
    recepies.insert(TileSelector::TechRefinery1, get_recepie(&TileSelector::TechRefinery1));
    recepies.insert(TileSelector::TechRefinery2, get_recepie(&TileSelector::TechRefinery2));
    recepies.insert(TileSelector::TechMarket, get_recepie(&TileSelector::TechMarket));
    recepies.insert(TileSelector::TechTurret1, get_recepie(&TileSelector::TechTurret1));
    recepies.insert(TileSelector::TechTurret2, get_recepie(&TileSelector::TechTurret2));
    recepies.insert(TileSelector::TechArtillery1, get_recepie(&TileSelector::TechArtillery1));
    recepies.insert(TileSelector::TechArtillery2, get_recepie(&TileSelector::TechArtillery2));
    recepies.insert(TileSelector::TechWall1, get_recepie(&TileSelector::TechWall1));
    recepies.insert(TileSelector::TechNuke, get_recepie(&TileSelector::TechNuke));

    recepies.insert(TileSelector::BugSoldierLV1, get_recepie(&TileSelector::BugSoldierLV1));
    recepies.insert(TileSelector::BugSoldierLV2, get_recepie(&TileSelector::BugSoldierLV2));
    recepies.insert(TileSelector::BugSoldierLV3, get_recepie(&TileSelector::BugSoldierLV3));
    recepies.insert(TileSelector::BugEliteMelee, get_recepie(&TileSelector::BugEliteMelee));
    recepies.insert(TileSelector::BugEliteRanged, get_recepie(&TileSelector::BugEliteRanged));

    recepies
}


pub fn get_recepie(selector: &TileSelector) -> TileRecepie {
    let stats = get_stats();
    let activation_costs = get_activation_costs();
    let footprints = get_footprints();
    let costs = get_costs();
    let spacings = get_spaced_placements();
    let roads = get_road_connection_requirements();
    TileRecepie {
        cost: costs.get(&selector).unwrap().clone(),
        footprint: footprints.get(&selector).unwrap().clone(),
        required_spaced_placement: spacings.get(&selector).unwrap().clone(),
        required_road_connection: roads.get(&selector).unwrap().clone(),
        stats: stats.get(&selector).unwrap().clone(),
        activated_costs: activation_costs.get(&selector).unwrap().clone(),
    }
}