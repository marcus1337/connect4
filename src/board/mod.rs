pub mod tile;
use std::fmt;
use tile::Tile;

use self::tile::Brick;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    OneWin,
    TwoWin,
    Draw,
    OnGoing,
}

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

    fn can_place_any(&mut self) -> bool {
        for col in 0..7 as usize {
            let num_col_bricks = self.get_num_col_bricks(col);
            if num_col_bricks < 6 {
                return true;
            }
        }
        false
    }

    fn is_line_of_4(line: [Tile; 4]) -> bool {
        match line[0] {
            Tile::Brick(_) => line[1..].iter().all(|tile| *tile == line[0]),
            _ => false,
        }
    }

    fn get_horizontal_lines(&self) -> Vec<[Tile; 4]> {
        let mut lines = Vec::<[Tile; 4]>::new();
        for col in 0..4 {
            for row in 0..6 {
                lines.push([
                    self.tiles[col][row],
                    self.tiles[col + 1][row],
                    self.tiles[col + 2][row],
                    self.tiles[col + 3][row],
                ]);
            }
        }
        lines
    }

    fn get_vertical_lines(&self) -> Vec<[Tile; 4]> {
        let mut lines = Vec::<[Tile; 4]>::new();
        for col in 0..7 {
            for row in 0..3 {
                lines.push([
                    self.tiles[col][row],
                    self.tiles[col][row + 1],
                    self.tiles[col][row + 2],
                    self.tiles[col][row + 3],
                ]);
            }
        }
        lines
    }

    fn get_diagonal_lines(&self) -> Vec<[Tile; 4]> {
        let mut lines = Vec::<[Tile; 4]>::new();
        for col in 0..4 {
            for row in 0..3 {
                lines.push([
                    self.tiles[col][row],
                    self.tiles[col+1][row + 1],
                    self.tiles[col+2][row + 2],
                    self.tiles[col+3][row + 3],
                ]);
                lines.push([
                    self.tiles[col][5-row],
                    self.tiles[col+1][5-row - 1],
                    self.tiles[col+2][5-row - 2],
                    self.tiles[col+3][5-row - 3],
                ]);
            }
        }
        lines
    }

    fn get_lines(&self) -> Vec<[Tile; 4]> {
        let mut lines = Vec::<[Tile; 4]>::new();
        lines.extend(self.get_horizontal_lines());
        lines.extend(self.get_vertical_lines());
        lines.extend(self.get_diagonal_lines());
        lines
    }

    fn get_line_of_4(&self) -> [Tile; 4] {
        for line in self.get_lines() {
            if Board::is_line_of_4(line) {
                return line;
            }
        }
        return [Tile::Empty; 4]
    }

    fn has_line_of_4(&self) -> bool {
        for line in self.get_lines() {
            if Board::is_line_of_4(line) {
                return true;
            }
        }
        false
    }

    #[no_mangle]
    pub extern "C" fn get_result(&mut self) -> GameResult {
        if self.has_line_of_4(){
            let win_line = self.get_line_of_4();
            if win_line[0] == Tile::Brick(Brick::One){
                return GameResult::OneWin;
            }else{
                return GameResult::TwoWin;
            }
        }
        if self.can_place_any() {
            return GameResult::OnGoing;
        }
        GameResult::Draw
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
            return Brick::One;
        } else {
            return Brick::Two;
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
    pub extern "C" fn can_place(&mut self, col: i32) -> bool {
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
