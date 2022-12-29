

pub enum Player{
    Player1,
    Player2,
}


pub struct Brick{
    pub player : Player,
}

#[derive(Copy, Clone)]
pub enum Tile{
    Empty,
    Brick,
}

