use std::collections::HashMap;

use crate::game::{game_models::types::{structure::{StructureSelector, Structure}, tile_traits::PassiveAbility, resource::Resouce}, core::game::GameState};

pub struct BugBase1Passive;
pub struct BugBase2Passive;
pub struct BugBase3Passive;
pub struct TechBasePassive;
pub struct TechMine1Passive;
pub struct TechMine2Passive;


pub fn get_passive_abilities() -> HashMap<StructureSelector, Option<Box<dyn PassiveAbility>>> {
    let mut hm: HashMap<StructureSelector, Option<Box<dyn PassiveAbility>>> = HashMap::new();
    hm.insert(StructureSelector::BugBase1, Some(Box::new(BugBase1Passive{})));
    hm.insert(StructureSelector::BugBase2, Some(Box::new(BugBase2Passive{})));
    hm.insert(StructureSelector::BugBase3, Some(Box::new(BugBase3Passive{})));
    hm.insert(StructureSelector::TechBase, Some(Box::new(TechBasePassive{})));
    hm.insert(StructureSelector::TechRoad, None);
    hm.insert(StructureSelector::TechMine1, Some(Box::new(TechMine1Passive{})));
    hm.insert(StructureSelector::TechMine2, Some(Box::new(TechMine2Passive{})));
    hm.insert(StructureSelector::TechRefinery1, None);
    hm.insert(StructureSelector::TechRefinery2, None);
    hm.insert(StructureSelector::TechMarket, None);
    hm.insert(StructureSelector::TechTurret1, None);
    hm.insert(StructureSelector::TechTurret2, None);
    hm.insert(StructureSelector::TechArtillery1, None);
    hm.insert(StructureSelector::TechArtillery2, None);
    hm.insert(StructureSelector::TechWall1, None);
    hm.insert(StructureSelector::TechNuke, None);
    hm
}

impl PassiveAbility for TechBasePassive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.tech_resources.push(Resouce::Gold);
        game_state.tech_resources.push(Resouce::Gold);
        game_state.tech_resources.push(Resouce::Gold);
        true
    }
}

impl PassiveAbility for TechMine1Passive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.tech_resources.push(Resouce::Gold);
        game_state.tech_resources.push(Resouce::Gold);
        true
    }
}

impl PassiveAbility for TechMine2Passive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.tech_resources.push(Resouce::Gold);
        game_state.tech_resources.push(Resouce::Gold);
        game_state.tech_resources.push(Resouce::Gold);
        true
    }
}

impl PassiveAbility for BugBase1Passive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.bug_resources.push(Resouce::Egg);
        true
    }
}

impl PassiveAbility for BugBase2Passive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.bug_resources.push(Resouce::Egg);
        game_state.bug_resources.push(Resouce::Egg);
        true
    }
}

impl PassiveAbility for BugBase3Passive {
    fn activate_passive(self, game_state: &mut GameState, structure: &mut Structure) -> bool {
        //TODO: bonus on map edge

        game_state.bug_resources.push(Resouce::Egg);
        game_state.bug_resources.push(Resouce::Egg);
        game_state.bug_resources.push(Resouce::Egg);
        true
    }
}
