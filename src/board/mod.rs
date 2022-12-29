
pub mod tile;

pub struct Board{
    tiles: tile::Tile,
}

impl Board {
    pub fn new() -> Self {
        Self { tiles: tile::Tile::Empty }
    }
}
