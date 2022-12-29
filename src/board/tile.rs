
#[derive(Copy, Clone, Debug)]
pub enum Player{
    Player1,
    Player2,
}

#[derive(Copy, Clone, Debug)]
pub struct Brick{
    pub player : Player,
}

#[derive(Copy, Clone, Debug)]
pub enum Tile{
    Empty,
    Brick(Brick),
}

impl Tile {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}


