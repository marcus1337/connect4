
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Brick{
    One, Two
}

impl Brick {
    pub fn get_opposite(&self) -> Brick {
        if *self == Brick::One{
            return Brick::Two;
        } else {
            return Brick::One;
        }
    }

    pub fn flip(&mut self) {
        *self = self.get_opposite();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile{
    Empty,
    Brick(Brick),
}

impl Tile {
    pub fn to_string(&self) -> String {
        let str = match self {
            Tile::Empty => "[ ]",
            Tile::Brick(Brick::One) => "[X]",
            Tile::Brick(Brick::Two) => "[O]"
        };
        String::from(str)
    }
}

use std::ops::{Add, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point{
    pub col: i32,
    pub row: i32,
}

impl Point{

    pub fn new(col: i32, row: i32) -> Self {
        Self{
            col:col, row:row
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.col >= 0 && self.col < 7 && self.row >= 0 && self.row < 6
    }

}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.col + other.col, self.row + other.row)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.col - other.col, self.row - other.row)
    }
}


