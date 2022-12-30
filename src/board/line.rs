

use super::tile;
use tile::Tile;
use tile::Brick;
use tile::Point;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub points: [Point; 4],
    pub tiles: [Tile; 4],
}

impl Line {

    pub fn make_default() -> Self{
        Self {
            points: [Point(0,0), Point(0,0), Point(0,0), Point(0,0)],
            tiles: [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
        }
    }

    pub fn is_win(&self) -> bool {
        match self.tiles[0] {
            Tile::Brick(_) => self.tiles[1..].iter().all(|tile| *tile == self.tiles[0]),
            _ => false,
        }
    }

    pub fn get_brick_type(&self) -> Brick {
        if let Tile::Brick(brick) = self.tiles[0] {
            return brick;
        }
        Brick::One
    }

    pub fn get_line_points() -> Vec<[Point; 4]> {
        let mut lines = Vec::<[Point; 4]>::new();
        lines.extend(Line::get_horizontal_line_points());
        lines.extend(Line::get_vertical_line_points());
        lines.extend(Line::get_diagonal_line_points());
        lines
    }

    pub fn get_horizontal_line_points() -> Vec<[Point; 4]> {
        let mut lines = Vec::<[Point; 4]>::new();
        for col in 0..4 {
            for row in 0..6 {
                lines.push([
                    Point(col, row),
                    Point(col + 1, row),
                    Point(col + 2, row),
                    Point(col + 3, row),
                ]);
            }
        }
        lines
    }

    pub fn get_vertical_line_points() -> Vec<[Point; 4]> {
        let mut lines = Vec::<[Point; 4]>::new();
        for col in 0..7 {
            for row in 0..3 {
                lines.push([
                    Point(col, row),
                    Point(col, row + 1),
                    Point(col, row + 2),
                    Point(col, row + 3),
                ]);
            }
        }
        lines
    }

    pub fn get_diagonal_line_points() -> Vec<[Point; 4]> {
        let mut lines = Vec::<[Point; 4]>::new();
        for col in 0..4 {
            for row in 0..3 {
                lines.push([
                    Point(col, row),
                    Point(col + 1, row + 1),
                    Point(col + 2, row + 2),
                    Point(col + 3, row + 3),
                ]);
                lines.push([
                    Point(col, 5 -row),
                    Point(col + 1, 5 - row - 1),
                    Point(col + 2, 5 - row - 2),
                    Point(col + 3, 5 - row - 3),
                ]);
            }
        }
        lines
    }

}

