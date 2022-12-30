
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Brick{
    One, Two
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

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub i32,pub i32);



