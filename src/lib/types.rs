//Data types
pub struct Cell {
    pub player: Player,
    pub status: CellStatus,
}

#[derive(Debug, PartialEq)]
pub enum CellStatus {
    Occupied,
    Free,
}

#[derive(Debug, PartialEq)]
pub enum Player {
    Blue,
    Red,
    Neutral,
}
pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub enum GameEvent {
    FieldOccupied,
    GameTied,
    GameWon,
}

pub struct CellColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
