// status pola na planszy
#[derive(Clone, Copy, PartialEq)]
pub enum Field {
    Water,
    Ship,
    Hit,
    Miss,
    Sunk,
}

// stan ogolny gry
#[derive(PartialEq)]
pub enum GameState {
    Placement,
    PlayerTurn,
    ComputerTurn,
    GameOver(String),
}
