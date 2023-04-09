//Data types
pub struct Cell {
    pub player: Player,
    pub status: CellStatus,
    // later :pixel_type: i32,
}

pub enum CellStatus {
    Occupied,
    Free,
}

pub enum Player {
    Blue,
    Red,
    Neutral,
}
pub struct Grid {
    /// Col<Row<Cell>>
    pub grid: Vec<Vec<Cell>>,
}


#[derive(Debug)]
pub enum GameError {
    ColumnFull,
}

pub struct CellColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct WinReg {
    owner: Player,
    diagonal_falling: [Field; 4],
    diagonal_rising: [Field; 4],
    up: [Field; 4],
    over: [Field; 4],
}

pub enum Field {
    Some((i32, i32)),
    None,
}