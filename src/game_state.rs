#[derive(PartialEq)]
pub enum GameState {
    Placement,
    PlayerTurn,
    ComputerTurn,
    GameOver(String),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Field {
    Water,
    Ship,
    Hit,
    Miss,
    Sunk,
}
