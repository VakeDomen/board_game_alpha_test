use std::{collections::HashMap, vec};

use crate::game::game_models::data::{recepies::get_recepie, spaced_placements::get_spaced_placements};

use super::{structure::{Structure, StructureSelector, NewStructure}, tile_traits::Tile};

pub type Map = Vec<Vec<String>>;


pub enum TileOption {
    Id(String), // tile id
    None,
    OutOfBounds,
}

type MapLocation = (usize, usize);


pub trait Extrcator {
    fn get_tile(&self, x: i32, y: i32) -> TileOption;
    fn get_tile_adjacent(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjacent_cornered(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjecent_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjecent_cornered_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_footprint_tiles(&self, x: i32, y: i32, footprint: &Vec<Vec<bool>>) -> Vec<(MapLocation, TileOption)>;
    fn get_footprint_tiles_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_adjacent_tiles(&self, x: i32, y: i32, directions: &[(i32, i32)]) -> Vec<(MapLocation, TileOption)>;
}

pub trait StructurePlacer {
    fn place_structure(&mut self, selector: NewStructure, tiles: HashMap<String, Tile>, x: i32, y: i32) -> Result<Structure, MapError> ;
    fn can_place_structure(&self, selector: &StructureSelector, tiles: HashMap<String, Tile>, x: i32, y: i32) -> Result<(), MapError>;
}

impl StructurePlacer for Map {
    fn place_structure(&mut self, mut structure: NewStructure, tiles: HashMap<String, Tile>, x: i32, y: i32) -> Result<Structure, MapError> {
        if let Err(e) = self.can_place_structure(&structure.structure_type, tiles, x, y) {
            return Err(e);
        }

        let recepie = get_recepie(&structure.structure_type);
        for (location, _) in self.get_footprint_tiles(x, y, &recepie.footprint) {
            self[location.0][location.1] = structure.id.to_string();
        }

        structure.x = Some(x);
        structure.y = Some(y);

        Ok(Structure::from(structure))
    } 

    fn can_place_structure(&self, selector: &StructureSelector, tiles: HashMap<String, Tile>, x: i32, y: i32) -> Result<(), MapError> {
        let recepie = get_recepie(selector);
        let footprint = self.get_footprint_tiles(x, y, &recepie.footprint);

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
                for (_, tile_option) in self.get_tile_adjacent(location.0 as i32, location.1 as i32) {
                    if let TileOption::Id(tile_id) = tile_option {
                        if let Some(tile) = tiles.get(&tile_id) {
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
                for (_, tile_option) in self.get_tile_adjacent(location.0 as i32, location.1 as i32) {
                    if let TileOption::Id(tile_id) = tile_option {
                        if let Some(tile) = tiles.get(&tile_id) {
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

impl Extrcator for Map {
    fn get_tile(&self, x: i32, y: i32) -> TileOption {
        if x >= self.len().try_into().unwrap() {
            return TileOption::OutOfBounds;
        }

        if y >= self.get(0).unwrap().len().try_into().unwrap() {
            return TileOption::OutOfBounds;
        }

        if self[x as usize][y as usize].is_empty() {
            TileOption::None
        } else {
            TileOption::Id(self[x as usize][y as usize].clone())
        }
    }

    // Returns the 4 tiles directly adjacent (up, down, left, right) to the specified location
    fn get_tile_adjacent(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Left, Right, Up, Down
        self.get_adjacent_tiles(x, y, &directions)
    }

    // Returns the 8 tiles surrounding the specified location, including diagonals
    fn get_tile_adjacent_cornered(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)> {
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]; // Including diagonals
        self.get_adjacent_tiles(x, y, &directions)
    }

    // Helper function to get adjacent tiles based on directions
    fn get_adjacent_tiles(&self, x: i32, y: i32, directions: &[(i32, i32)]) -> Vec<(MapLocation, TileOption)> {
        let mut tiles = Vec::new();
        for &(dx, dy) in directions {
            let adj_x = x + dx;
            let adj_y = y + dy;
            if adj_x >= 0 && adj_x < self[0].len() as i32 && adj_y >= 0 && adj_y < self.len() as i32 {
                let location = (adj_x as usize, adj_y as usize);
                let tile_option = match self[adj_y as usize][adj_x as usize].as_str() {
                    "" => TileOption::None,
                    id => TileOption::Id(id.to_string()),
                };
                tiles.push((location, tile_option));
            } else {
                tiles.push(((adj_x as usize, adj_y as usize), TileOption::OutOfBounds));
            }
        }
        tiles
    }


    fn get_tile_adjecent_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)> {
        let mut location_x = None;
        let mut location_y = None;

        for (index, row) in self.iter().enumerate() {
            for (inner_index, col) in row.iter().enumerate() {
                if col.eq(id) {
                    location_x = Some(index);
                    location_y = Some(inner_index);
                    break;
                }
            }
        }
        if location_x.is_none() || location_y.is_none() {
            return vec![];
        }

        let location_x = location_x.unwrap();
        let location_y = location_y.unwrap();
    
        self.get_tile_adjacent(location_x as i32, location_y as i32)
    }

    fn get_tile_adjecent_cornered_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)> {
        let mut location_x = None;
        let mut location_y = None;

        for (index, row) in self.iter().enumerate() {
            for (inner_index, col) in row.iter().enumerate() {
                if col.eq(id) {
                    location_x = Some(index);
                    location_y = Some(inner_index);
                    break;
                }
            }
        }
        if location_x.is_none() || location_y.is_none() {
            return vec![];
        }

        let location_x = location_x.unwrap();
        let location_y = location_y.unwrap();
    
        self.get_tile_adjacent_cornered(location_x as i32, location_y as i32)
    }

    fn get_footprint_tiles(&self, x: i32, y: i32, footprint: &Vec<Vec<bool>>) -> Vec<(MapLocation, TileOption)> {
        let mut tiles: Vec<(MapLocation, TileOption)> = Vec::new();

        for (dy, row) in footprint.iter().enumerate() {
            for (dx, &part_of_footprint) in row.iter().enumerate() {
                // Calculate the actual x and y coordinates on the map
                let current_x = x as usize + dx;
                let current_y = y as usize + dy;

                // Determine the map location and tile option
                let location = (current_x, current_y);
                let tile_option = if 
                    current_x >= self.len() || 
                    current_y >= self[0].len() 
                {
                    TileOption::OutOfBounds
                } else if part_of_footprint {
                    match self[current_x][current_y].as_str() {
                        "" => TileOption::None,
                        id => TileOption::Id(id.to_string()),
                    }
                } else {
                    continue; // If the footprint is false, we don't need to record the tile
                };

                tiles.push((location, tile_option));
            }
        }

        tiles
    }

    fn get_footprint_tiles_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)> {
        let mut out = vec![];
        for (index, row) in self.iter().enumerate() {
            for (inner_index, col) in row.iter().enumerate() {
                if col.eq(id) {
                    out.push(((index, inner_index), TileOption::Id(id.clone())));
                }
            }
        }
        out
    }

}

#[derive(Debug)]
pub enum MapError {
    ContructionObstructed,
    NotConnectedToRoad,
    NotEnougyProximitySpace,
    ConstructionOutOfBounds,
}