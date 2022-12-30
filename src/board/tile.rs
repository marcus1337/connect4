
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Brick{
    ONE, TWO
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
            Tile::Brick(Brick::ONE) => "[X]",
            Tile::Brick(Brick::TWO) => "[O]"
        };
        String::from(str)
    }
}


