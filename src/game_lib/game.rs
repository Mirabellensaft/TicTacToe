use crate::game_lib::types::{Cell, CellStatus, GameEvent, Grid, Player};

/// This function updates the grid with the new mark and also calls a bunch of other functions
/// that manage game events.
pub fn update_grid_with_new_mark(
    grid: &mut Grid,
    column: i32,
    row: i32,
    player: &Player,
) -> Result<(), GameEvent> {
    field_occupied(grid, column, row)?;

    let cell = match player {
        Player::Blue => Cell {
            player: Player::Blue,
            status: CellStatus::Occupied,
        },
        Player::Red => Cell {
            player: Player::Red,
            status: CellStatus::Occupied,
        },
        Player::Neutral => Cell {
            player: Player::Neutral,
            status: CellStatus::Free,
        },
    };

    grid.grid[column as usize][row as usize] = cell;
    // println!("grid updated");
    detect_win(grid, player, row, column)?;
    grid_full(&grid)?;
    Ok(())
}

/// This function switches to the next player
pub fn switch_player(player: Player) -> Player {
    let new_player = match player {
        Player::Blue => Player::Red,
        Player::Red => Player::Blue,
        Player::Neutral => player,
    };
    println!("switched Player");

    new_player
}

/// This function checks if a field is occupied
fn field_occupied(grid: &Grid, column: i32, row: i32) -> Result<(), GameEvent> {
    match &grid.grid[column as usize][row as usize].status {
        CellStatus::Occupied => {
            // println!("occupied");

            return Err(GameEvent::FieldOccupied);
        }
        CellStatus::Free => {
            // println!("free");
            return Ok(());
        }
    }
}

/// This function checks if all marks are placed.
fn grid_full(grid: &Grid) -> Result<(), GameEvent> {
    let max_rows = grid.grid.len();
    let max_columns = grid.grid[0].len();

    for column in 0..max_columns {
        for _row in 0..max_rows {
            if grid.grid[column][_row].status == CellStatus::Free {
                return Ok(());
            }
        }
    }
    Err(GameEvent::GameTied)
}

/// This function calls some other functions that detect if someone has won.
fn detect_win(
    grid: &Grid,
    player: &Player,
    last_row: i32,
    last_column: i32,
) -> Result<(), GameEvent> {
    across_win(grid, last_row, last_column, player)?;
    diagonal_win(grid, player)?;
    Ok(())
}

/// Checks if there are three equal marks in straight lines
fn across_win(
    grid: &Grid,
    last_row: i32,
    last_column: i32,
    player: &Player,
) -> Result<(), GameEvent> {
    let max_rows = grid.grid[0].len();
    let max_columns = grid.grid.len();

    let mut counter_across = 0;
    let mut counter_down = 0;

    for column in 0..max_columns {
        if grid.grid[column][last_row as usize].status == CellStatus::Occupied
            && grid.grid[column][last_row as usize].player == *player
        {
            counter_across += 1;
        }
    }

    for row in 0..max_rows {
        if grid.grid[last_column as usize][row].player == *player {
            counter_down += 1;
        }
    }

    if counter_across == 3 || counter_down == 3 {
        return Err(GameEvent::GameWon);
    } else {
        return Ok(());
    }
}

/// Checks if any of the diagonals are occupied by the same player
fn diagonal_win(grid: &Grid, player: &Player) -> Result<(), GameEvent> {
    let rise = [(0, 0), (1, 1), (2, 2)];
    let fall = [(0, 2), (1, 1), (2, 0)];

    let diagonals = [rise, fall];

    for diagonal in diagonals {
        let mut counter = 0;
        for coordinate in 0..3 {
            if grid.grid[diagonal[coordinate].0 as usize][diagonal[coordinate].1 as usize].player
                == *player
            {
                counter += 1;
            }
        }

        if counter == 3 {
            return Err(GameEvent::GameWon);
        }
    }
    Ok(())
}
