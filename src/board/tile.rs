

pub enum Player{
    Player1,
    Player2,
}


pub struct Brick{
    pub player : Player,
}

#[derive(Copy, Clone, Debug)]
pub enum Tile{
    Empty,
    Brick,
}

impl Tile {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}


