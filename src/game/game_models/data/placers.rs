use std::collections::HashMap;

use crate::game::{game_models::{types::{tile::{TileSelector, Tile, NewTile}, tile_traits::Placable, map::{MapError, TileOption, Interactor}}, functions::ability_active::remove_resources}, core::game_state::GameState};

use super::{recepies::get_recepie, spaced_placements::get_spaced_placements, footprints::get_footprints, costs::get_costs};



pub struct BasicPlacer;
pub struct BasicBugPlacer;


pub fn get_placers() -> HashMap<TileSelector, Option<Box<dyn Placable>>> {
    let mut hm: HashMap<TileSelector, Option<Box<dyn Placable>>> = HashMap::new();
    hm.insert(TileSelector::BugBase1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::BugBase2, None);
    hm.insert(TileSelector::BugBase3, None);
    hm.insert(TileSelector::TechBase, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechRoad, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechMine1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechMine2, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechRefinery1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechRefinery2, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechMarket, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechTurret1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechTurret2, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechArtillery1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechArtillery2, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechWall1, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::TechNuke, Some(Box::new(BasicPlacer{})));
    hm.insert(TileSelector::BugSoldierLV1, Some(Box::new(BasicBugPlacer{})));
    hm.insert(TileSelector::BugSoldierLV2, Some(Box::new(BasicBugPlacer{})));
    hm.insert(TileSelector::BugSoldierLV3, Some(Box::new(BasicBugPlacer{})));
    hm.insert(TileSelector::BugEliteMelee, Some(Box::new(BasicBugPlacer{})));
    hm.insert(TileSelector::BugEliteRanged, Some(Box::new(BasicBugPlacer{})));
    hm
}


impl Placable for BasicBugPlacer {
    fn place(&self, mut tile: NewTile, game_state: &mut GameState, x: i32, y: i32) -> Result<Tile, MapError> {
        if let Err(e) = self.can_place_on(&tile, game_state, x, y) {
            return Err(e);
        }

        
        let costs = get_costs();
        let cost = costs.get(&tile.tile_type).unwrap();
        let footprints = get_footprints();
        let footprint = footprints.get(&tile.tile_type).unwrap();

        let fp_tiles = if tile.rotated {
            game_state.map.get_rotated_footprint_tiles(x, y, footprint)
        } else {
            game_state.map.get_footprint_tiles(x, y, footprint)
        };
        
        for (location, _) in fp_tiles {
            game_state.map[location.0][location.1] = tile.id.to_string();
        }

        tile.x = Some(x);
        tile.y = Some(y);


        if !remove_resources(&mut game_state.bug_resources, cost) {
            return Err(MapError::NotEnoughResources);
        }

        Ok(Tile::from(tile))

    }

    fn can_place_on(&self, tile: &NewTile, game_state: &GameState, x: i32, y: i32) -> Result<(), MapError> {


        let footprints = get_footprints();
        let footprint = footprints.get(&tile.tile_type).unwrap();

        let fp_tiles = if tile.rotated {
            game_state.map.get_rotated_footprint_tiles(x, y, footprint)
        } else {
            game_state.map.get_footprint_tiles(x, y, footprint)
        };
        
        for (current_check_location, _) in fp_tiles {
            if !game_state.map[current_check_location.0][current_check_location.1].is_empty() {
                return Err(MapError::ContructionObstructed);
            }
            for (_, tile_option) in game_state.map.get_tile_adjacent_cornered(current_check_location.0 as i32, current_check_location.1 as i32) {
                if let TileOption::Id(id) = tile_option {
                    let neighbour = game_state.tiles.get(&id).unwrap();
                    if 
                        neighbour.tile_type == TileSelector::BugBase1 ||
                        neighbour.tile_type == TileSelector::BugBase2 ||
                        neighbour.tile_type == TileSelector::BugBase3 
                    {
                        return Ok(())
                    }
                }
            }
        }
        Err(MapError::NotConnectedToNest)
    }
}


impl Placable for BasicPlacer {
    fn place(&self, mut tile: NewTile, game_state: &mut GameState, x: i32, y: i32) -> Result<Tile, MapError> {
        if let Err(e) = self.can_place_on(&tile, &game_state, x, y) {
            return Err(e);
        }

        let recepie = get_recepie(&tile.tile_type);
        for (location, _) in game_state.map.get_footprint_tiles(x, y, &recepie.footprint) {
            game_state.map[location.0][location.1] = tile.id.to_string();
        }

        
        tile.x = Some(x);
        tile.y = Some(y);

        Ok(Tile::from(tile))
    } 

    fn can_place_on(&self, tile: &NewTile, game_state: &GameState, x: i32, y: i32) -> Result<(), MapError> {

        let recepie = get_recepie(&tile.tile_type);
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
                            if tile.tile_type == TileSelector::TechRoad {
                                found_road = true;
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
                            if *spacings.get(&tile.tile_type).unwrap() {
                                naighbouring_space_occupied = true;
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