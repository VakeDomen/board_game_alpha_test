pub type Map = Vec<Vec<String>>;


pub enum TileOption {
    Some(String), // tile id
    None,
    OutOfBounds,
}

pub trait Extrcator {
    fn get_tile(&self, x: i32, y: i32) -> TileOption;
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
            TileOption::Some(self[x as usize][y as usize].clone())
        }
    }
}