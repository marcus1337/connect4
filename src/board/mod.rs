pub mod tile;
use tile::Tile;

pub struct Board {
    tiles: [[Tile; 7]; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 7]; 6],
        }
    }
}
