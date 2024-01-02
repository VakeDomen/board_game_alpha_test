use std::collections::HashMap;

use crate::game::{game_models::{types::{unit::{UnitSelector, Unit}, tile_traits::{Placable, Tile, NewTile}, map::{MapError, TileOption, Interactor}, structure::StructureSelector}, data::structures::{recepies::get_recepie, spaced_placements::get_spaced_placements}, functions::ability_active::remove_resources}, core::game_state::GameState};

use super::{costs::get_costs, footprints::{get_footprints, self}};


pub struct BasicBugPlacer;

pub fn get_placers() -> HashMap<UnitSelector, Option<Box<dyn Placable>>> {
    let mut hm: HashMap<UnitSelector, Option<Box<dyn Placable>>> = HashMap::new();
    hm.insert(UnitSelector::BugSoldierLV1, Some(Box::new(BasicBugPlacer{})));
    hm.insert(UnitSelector::BugSoldierLV2, Some(Box::new(BasicBugPlacer{})));
    hm.insert(UnitSelector::BugSoldierLV3, Some(Box::new(BasicBugPlacer{})));
    hm.insert(UnitSelector::BugEliteMelee, Some(Box::new(BasicBugPlacer{})));
    hm.insert(UnitSelector::BugEliteRanged, Some(Box::new(BasicBugPlacer{})));
    hm
}

impl Placable for BasicBugPlacer {
    fn place(&self, new_tile: NewTile, game_state: &mut GameState, x: i32, y: i32) -> Result<Tile, MapError> {
        if let Err(e) = self.can_place_on(&new_tile, game_state, x, y) {
            return Err(e);
        }

        let mut unit = match new_tile {
            NewTile::Unit(u) => u,
            NewTile::Structure(s) => return Err(MapError::IncorrectPlacer),
        };
        
        let costs = get_costs();
        let cost = costs.get(&unit.unit_type).unwrap();
        let footprints = get_footprints();
        let footprint = footprints.get(&unit.unit_type).unwrap();

        let fp_tiles = if unit.rotated {
            game_state.map.get_rotated_footprint_tiles(x, y, footprint)
        } else {
            game_state.map.get_footprint_tiles(x, y, footprint)
        };
        
        for (location, _) in fp_tiles {
            game_state.map[location.0][location.1] = unit.id.to_string();
        }

        unit.x = Some(x);
        unit.y = Some(y);


        if !remove_resources(&mut game_state.bug_resources, cost) {
            return Err(MapError::NotEnoughResources);
        }

        let unit = Unit::from(unit);
        Ok(Tile::Unit(unit))

    }

    fn can_place_on(&self, new_tile: &NewTile, game_state: &GameState, x: i32, y: i32) -> Result<(), MapError> {
        let unit = match new_tile {
            NewTile::Unit(u) => u,
            NewTile::Structure(s) => return Err(MapError::IncorrectPlacer),
        };

        let footprints = get_footprints();
        let footprint = footprints.get(&unit.unit_type).unwrap();

        let fp_tiles = if unit.rotated {
            game_state.map.get_rotated_footprint_tiles(x, y, footprint)
        } else {
            game_state.map.get_footprint_tiles(x, y, footprint)
        };
        
        for (current_check_location, _) in fp_tiles {
            if !game_state.map[current_check_location.0][current_check_location.1].is_empty() {
                return Err(MapError::ContructionObstructed);
            }
            for (location, tile_option) in game_state.map.get_tile_adjacent_cornered(current_check_location.0 as i32, current_check_location.1 as i32) {
                if let TileOption::Id(id) = tile_option {
                    match game_state.tiles.get(&id).unwrap() {
                        Tile::Structure(s) => {
                            if 
                                s.structure_type == StructureSelector::BugBase1 ||
                                s.structure_type == StructureSelector::BugBase2 ||
                                s.structure_type == StructureSelector::BugBase3 
                            {
                                return Ok(())
                            }
                        },
                        Tile::Unit(u) => return Ok(()),
                    }
                }
            }
        }
        Err(MapError::NotConnectedToNest)
    }
}
