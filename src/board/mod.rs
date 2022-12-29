pub mod tile;
use tile::Tile;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub tiles: [[Tile; 7]; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 7]; 6],
        }
    }

    pub fn reset(&mut self){
        *self = Board::new();
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board_string = self.tiles
            .iter()
            .map(|row| row.iter().map(|tile| tile.to_string()).collect::<Vec<String>>().join(" "))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", board_string)
    }
}

