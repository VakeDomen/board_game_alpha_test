use super::{
    tiles::Upgradable, 
    structure::{Structure, StructureSelector}, resource::Resouce
};


pub struct TurretUpgrader;

impl Upgradable for TurretUpgrader {
    fn upgrade(self, game_state: crate::game::core::game::GameState, structure: &mut Structure) -> Option<Structure> {
        match structure.structure_type {
            StructureSelector::TechTurret1 => (),
            StructureSelector::TechArtillery1 => (),
            _ => return None,
        };


        if structure.activation_resources != vec![Resouce::Metal] {
            return None;
        }
        
        structure.activation_resources = vec![];
        
        match structure.structure_type {
            StructureSelector::TechTurret1 => structure.structure_type = StructureSelector::TechTurret2,
            StructureSelector::TechArtillery1 => structure.structure_type = StructureSelector::TechArtillery2,
            _ => (),
        }


        None
    }

    fn can_upgrade(self, game_state: crate::game::core::game::GameState, structure: &mut Structure, x: i32, y: i32) -> bool {
        todo!()
    }

    fn has_enough_resources_for_upgrade(self, game_state: crate::game::core::game::GameState, structure: &mut Structure) -> bool {
        todo!()
    }
}