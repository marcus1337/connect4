pub mod tile;
use tile::Tile;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum GameType{
    ConnectFour
}

#[derive(Debug)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub board_type: GameType,
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

impl Board {
    pub fn new(board_type: GameType) -> Self {

        match board_type {
            GameType::ConnectFour => Self {
                board_type: board_type,
                tiles: vec![vec![Tile::Empty; 7]; 6],
            }
        }

    }

    pub fn reset(&mut self){
        *self = Board::new(self.board_type);
    }
}
