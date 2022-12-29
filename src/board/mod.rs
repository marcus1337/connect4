pub mod tile;
use std::fmt;
use tile::Tile;

use self::tile::Brick;
use self::tile::Player;

#[derive(Debug)]
pub struct Board {
    pub tiles: [[Tile; 6]; 7],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; 6]; 7],
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }

    fn get_num_col_bricks(&mut self, col: usize) -> usize {
        let mut num_bricks = 0;
        for row in 0..6 {
            if let Tile::Brick(_) = self.tiles[col][row] {
                num_bricks += 1;
            }
        }
        num_bricks
    }

    fn get_num_bricks(&mut self) -> i32 {
        let count_bricks = self
            .tiles
            .iter()
            .flatten()
            .filter(|x| match x {
                Tile::Brick(_) => true,
                _ => false,
            })
            .count();
        count_bricks as i32
    }

    pub fn get_player(&mut self) -> Player {
        if self.get_num_bricks() % 2 == 0 {
            return Player::Player1;
        } else {
            return Player::Player2;
        };
    }

    fn get_place_point(&mut self, col: usize) -> Option<(usize, usize)> {
        if col >= 7 {
            return None;
        }
        let row = self.get_num_col_bricks(col);
        if row >= 6 {
            return None;
        }
        return Some((col, row));
    }

    fn get_tile_update(&mut self) -> Tile{
        let player = self.get_player();
        let brick = Brick { player: player };
        let tile = Tile::Brick(brick);
        tile
    }

    pub fn place(&mut self, col: usize) {
        if let Some((col, row)) = self.get_place_point(col) {
            self.tiles[col][row] = self.get_tile_update();
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = "".to_owned();
        for row in 0..6 {
            for col in 0..7 {
                board_string += self.tiles[col][5 - row].to_string().as_str();
            }
            board_string += "\n";
        }
        write!(f, "{}", board_string)
    }
}
