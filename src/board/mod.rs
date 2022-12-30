pub mod tile;
pub mod line;

use std::fmt;
use tile::Point;
use tile::Tile;
use line::Line;

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

    fn get_tile(&self, point: Point) -> Tile {
        let col = point.0 as usize;
        let row = point.1 as usize;
        self.tiles[col][row]
    }

    fn get_line(&self, points: [Point; 4]) -> Line{
        let tile1 = self.get_tile(points[0]);
        let tile2 = self.get_tile(points[1]);
        let tile3 = self.get_tile(points[2]);
        let tile4 = self.get_tile(points[3]);
        Line{
            tiles: [tile1, tile2, tile3, tile4],
            points: points
        }
    }


    fn get_lines(&self) -> Vec<Line> {
        let mut lines = Vec::<Line>::new();
        for line_points in Line::get_line_points() {
            lines.push(self.get_line(line_points));
        }
        lines
    }

    #[no_mangle]
    pub extern "C" fn get_win_line(&self) -> Line {
        for line in self.get_lines() {
            if line.is_win() {
                return line;
            }
        }
        return Line::make_default();
    }

    fn has_line_of_4(&self) -> bool {
        for line in self.get_lines() {
            if line.is_win() {
                return true;
            }
        }
        false
    }

    #[no_mangle]
    pub extern "C" fn get_result(&mut self) -> GameResult {
        if self.has_line_of_4() {
            let win_line = self.get_win_line();
            if win_line.get_brick_type() == Brick::One {
                return GameResult::OneWin;
            } else {
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

    #[no_mangle]
    pub extern "C" fn get_next_brick(&mut self) -> Brick {
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
