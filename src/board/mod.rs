pub mod tile;
use std::fmt;
use tile::Tile;

use self::tile::Brick;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

    pub fn get_next_brick(&mut self) -> Brick {
        if self.get_num_bricks() % 2 == 0 {
            return Brick::ONE;
        } else {
            return Brick::TWO;
        };
    }

    fn get_tile_update(&mut self) -> Tile {
        let brick = self.get_next_brick();
        let tile = Tile::Brick(brick);
        tile
    }

    #[no_mangle]
    pub extern "C" fn has_brick(&mut self, col: i32, row: i32) -> bool {
        let tile = self.tiles[col as usize][row as usize];
        if let Tile::Brick(_) = tile {
            return true;
        }
        false
    }

    #[no_mangle]
    pub extern "C" fn is_brick(&mut self, brick: Brick, col: i32, row: i32) -> bool {
        let tile = self.tiles[col as usize][row as usize];
        if let Tile::Brick(tile_brick) = tile {
            return tile_brick == brick;
        }
        false
    }

    #[no_mangle]
    pub extern "C" fn canPlace(&mut self, col: i32) -> bool {
        let row = self.get_num_col_bricks(col as usize);
        row < 6
    }

    #[no_mangle]
    pub extern "C" fn place(&mut self, col: i32) {
        let row = self.get_num_col_bricks(col as usize);
        self.tiles[col as usize][row] = self.get_tile_update();
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
