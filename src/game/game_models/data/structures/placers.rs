use std::collections::HashMap;

use crate::game::{game_models::{types::{structure::{StructureSelector, Structure}, tile_traits::{Placable, Tile, NewTile}, map::{MapError, TileOption, Interactor}}, data::structures::{recepies::get_recepie, spaced_placements::get_spaced_placements}}, core::game_state::GameState};


pub struct BasicPlacer;


pub fn get_placers() -> HashMap<StructureSelector, Option<Box<dyn Placable>>> {
    let mut hm: HashMap<StructureSelector, Option<Box<dyn Placable>>> = HashMap::new();
    hm.insert(StructureSelector::BugBase1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::BugBase2, None);
    hm.insert(StructureSelector::BugBase3, None);
    hm.insert(StructureSelector::TechBase, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechRoad, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechMine1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechMine2, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechRefinery1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechRefinery2, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechMarket, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechTurret1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechTurret2, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechArtillery1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechArtillery2, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechWall1, Some(Box::new(BasicPlacer{})));
    hm.insert(StructureSelector::TechNuke, Some(Box::new(BasicPlacer{})));
    hm
}

impl Placable for BasicPlacer {
    fn place(&self, new_tile: NewTile, game_state: &mut GameState, x: i32, y: i32) -> Result<Tile, MapError> {
        if let Err(e) = self.can_place_on(&new_tile, &game_state, x, y) {
            return Err(e);
        }

        let mut structure = match new_tile {
            NewTile::Structure(s) => s.clone(),
            _ => return Err(MapError::IncorrectPlacer),
        };

        let recepie = get_recepie(&structure.structure_type);
        for (location, _) in game_state.map.get_footprint_tiles(x, y, &recepie.footprint) {
            game_state.map[location.0][location.1] = structure.id.to_string();
        }

        
        structure.x = Some(x);
        structure.y = Some(y);

        Ok(Tile::Structure(Structure::from(structure)))
    } 

    fn can_place_on(&self, new_tile: &NewTile, game_state: &GameState, x: i32, y: i32) -> Result<(), MapError> {
        let structure = if let NewTile::Structure(s) = new_tile {
            s
        } else {
            return Err(MapError::IncorrectPlacer);
        };
        
        let recepie = get_recepie(&structure.structure_type);
        let footprint = game_state.map.get_footprint_tiles(x, y, &recepie.footprint);

        // are tiles free?
        for (_, tile) in footprint.iter() {
            match tile {
                TileOption::Id(_) => return Err(MapError::ContructionObstructed),
                TileOption::OutOfBounds => return Err(MapError::ConstructionOutOfBounds),
                _ => (),
            };
        }

        // is it connected to a road if it needs to be?
        if recepie.required_road_connection {
            let mut found_road = false;

            for (location, _) in footprint.iter() {
                for (_, tile_option) in game_state.map.get_tile_adjacent(location.0 as i32, location.1 as i32) {
                    if let TileOption::Id(tile_id) = tile_option {
                        if let Some(tile) = game_state.tiles.get(&tile_id) {
                            if let Tile::Structure(s) = tile {
                                if s.structure_type == StructureSelector::TechRoad {
                                    found_road = true;
                                }
                            }
                        }
                    }
                }
            }

            if !found_road {
                return Err(MapError::NotConnectedToRoad)
            }
        }

        // is is dufficiently far from other buildings?
        if recepie.required_spaced_placement {
            let mut naighbouring_space_occupied = false;
            let spacings = get_spaced_placements();
            for (location, _) in footprint.iter() {
                for (_, tile_option) in game_state.map.get_tile_adjacent(location.0 as i32, location.1 as i32) {
                    if let TileOption::Id(tile_id) = tile_option {
                        if let Some(tile) = game_state.tiles.get(&tile_id) {
                            if let Tile::Structure(s) = tile {
                                if *spacings.get(&s.structure_type).unwrap() {
                                    naighbouring_space_occupied = true;
                                }
                            }
                        }
                    }
                }
            }

            if !naighbouring_space_occupied {
                return Err(MapError::NotEnougyProximitySpace)
            }
        }

        Ok(())
    }
}