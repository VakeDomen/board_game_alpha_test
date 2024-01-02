pub type Map = Vec<Vec<String>>;


pub enum TileOption {
    Id(String), // tile id
    None,
    OutOfBounds,
}

type MapLocation = (usize, usize);


pub trait Interactor {
    fn get_tile(&self, x: i32, y: i32) -> TileOption;
    fn get_tile_adjacent(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjacent_cornered(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_corners(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjecent_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_tile_adjecent_cornered_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_footprint_tiles(&self, x: i32, y: i32, footprint: &Vec<Vec<bool>>) -> Vec<(MapLocation, TileOption)>;
    fn get_rotated_footprint_tiles(&self, x: i32, y: i32, footprint: &Vec<Vec<bool>>) -> Vec<(MapLocation, TileOption)>;
    fn get_footprint_tiles_by_id(&self, id: &String) -> Vec<(MapLocation, TileOption)>;
    fn get_adjacent_tiles(&self, x: i32, y: i32, directions: &[(i32, i32)]) -> Vec<(MapLocation, TileOption)>;

    fn remove_tile(&mut self, id: String) -> bool;
}

impl Interactor for Map {
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

    // Returns the 8 tiles surrounding the specified location, including diagonals
    fn get_tile_corners(&self, x: i32, y: i32) -> Vec<(MapLocation, TileOption)> {
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)]; // just diagonals
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


    fn get_rotated_footprint_tiles(&self, x: i32, y: i32, footprint: &Vec<Vec<bool>>) -> Vec<(MapLocation, TileOption)> {
        let mut tiles: Vec<(MapLocation, TileOption)> = Vec::new();

        for (dy, row) in footprint.iter().enumerate() {
            for (dx, &part_of_footprint) in row.iter().enumerate() {
                // Calculate the actual x and y coordinates on the map
                let current_x = x as usize + dy;
                let current_y = y as usize + dx;

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

    fn remove_tile(&mut self, id: String) -> bool {
        let mut found = false;
        for row in self {
            for col in row {
                if *col == id {
                    *col = "".to_string();
                    found = true;
                }
            }
        }
        found
    }

}

#[derive(Debug)]
pub enum MapError {
    ContructionObstructed,
    NotConnectedToRoad,
    NotEnougyProximitySpace,
    ConstructionOutOfBounds,
    IncorrectPlacer,
    NotConnectedToNest,
    NotEnoughResources,
}