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
            match self.tiles[col][row] {
                Tile::Brick(brick) => num_bricks += 1,
                _ => (),
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
                Tile::Brick(brick) => true,
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

    pub fn place(&mut self, col: usize) {
        let row = self.get_num_col_bricks(col);
        if row >= 6 || col >= 7{
            return;
        }

        let player = self.get_player();
        let brick = Brick { player: player };
        let tile = Tile::Brick(brick);
        self.tiles[col][row] = tile;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = "".to_owned();
        for row in 0..6 {
            for col in 0..7 {
                match self.tiles[col][5 - row] {
                    Tile::Brick(Brick {
                        player: Player::Player1,
                    }) => board_string += "[X]",
                    Tile::Brick(Brick {
                        player: Player::Player2,
                    }) => board_string += "[O]",
                    _ => board_string += "[ ]",
                }
            }
            board_string += "\n";
        }
        write!(f, "{}", board_string)
    }
}
