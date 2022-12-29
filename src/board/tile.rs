
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player{
    Player1,
    Player2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Brick{
    pub player : Player,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile{
    Empty,
    Brick(Brick),
}

impl Tile {
    pub fn to_string(&self) -> String {
        let str = match self {
            Tile::Empty => "[ ]",
            Tile::Brick(Brick{player: Player::Player1}) => "[X]",
            Tile::Brick(Brick{player: Player::Player2}) => "[O]"
        };
        String::from(str)
    }
}


