/// A Cell is one of the 9 squares that need to be colored to set a player's mark.
/// It keeps track if the cell is occupied or not, and if yes, by which player.
pub struct Cell {
    pub player: Player,
    pub status: CellStatus,
}

/// CellStatus has two variants: Free or Occupied.
#[derive(Debug, PartialEq)]
pub enum CellStatus {
    Occupied,
    Free,
}

/// Player is either Blue, Red or Neutral for fields that aren't yet occupied.
#[derive(Debug, PartialEq)]
pub enum Player {
    Blue,
    Red,
    Neutral,
}
/// Grid is the arrangement of the Cells
pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}
/// GameEvent contains the variants of important game events, such as a field is clicked that is already occupied,
/// a game that is ended by a tie, and a game that is ended by one player winning.
#[derive(Debug)]
pub enum GameEvent {
    FieldOccupied,
    GameTied,
    GameWon,
}

/// CellColor keeps track of the Cell's color.
pub struct CellColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
