use std::collections::HashMap;

use super::structure::{Structure, StructureRecepie, StructureSelector};

pub type Map = Vec<Vec<String>>;


pub enum TileOption {
    Id(String), // tile id
    None,
    OutOfBounds,
}

type MapLocation = (usize, usize);


pub trait Extrcator {
    fn get_tile(&self, x: i32, y: i32) -> TileOption;
    fn get_tile_adejecent(&self, x: i32, y: i32) -> HashMap<MapLocation, TileOption>;
    fn get_tile_adejecent_cornered(&self, x: i32, y: i32) -> HashMap<MapLocation, TileOption>;
    fn get_tile_adejecent_by_id(&self, id: &String) -> HashMap<MapLocation, TileOption>;
    fn get_tile_adejecent_cornered_by_id(&self, id: &String) -> HashMap<MapLocation, TileOption>;
}

pub trait StructurePlacer {
    fn place_structure(&mut self, structure: &StructureSelector, x: i32, y: i32) -> Result<Structure, MapError> {
        todo!()
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

    fn get_tile_adejecent(&self, x: i32, y: i32) -> HashMap<MapLocation, TileOption> {
        todo!()
    }

    fn get_tile_adejecent_cornered(&self, x: i32, y: i32) -> HashMap<MapLocation, TileOption> {
        todo!()
    }

    fn get_tile_adejecent_by_id(&self, id: &String) -> HashMap<MapLocation, TileOption> {
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
            return HashMap::new();
        }

        let location_x = location_x.unwrap();
        let location_y = location_y.unwrap();
    
        self.get_tile_adejecent(location_x as i32, location_y as i32)
    }

    fn get_tile_adejecent_cornered_by_id(&self, id: &String) -> HashMap<MapLocation, TileOption> {
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
            return HashMap::new();
        }

        let location_x = location_x.unwrap();
        let location_y = location_y.unwrap();
    
        self.get_tile_adejecent_cornered(location_x as i32, location_y as i32)
    }

}

pub enum MapError {

}