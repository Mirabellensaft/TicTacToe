

use crate::lib::types::{Cell, CellStatus, Grid, GameError, Player};
pub fn update_grid_with_new_coin(grid: &mut Grid, column: i32, row: i32, player: &Player) -> Result<(), GameError> {
    let row = gravity_basic(grid, column, row)?;

    let cell = match player {
        Player::Blue => {
            Cell {
                player: Player::Blue,
                status: CellStatus::Occupied,
            }
        },
        Player::Red => {
            Cell {
                player: Player::Red,
                status: CellStatus::Occupied,
            }
        },
        Player::Neutral => {
            Cell {
                player: Player::Neutral,
                status: CellStatus::Free,
            }
        },
    };

    grid.grid[column as usize][row as usize] = cell;
    println!("grid updated");
    Ok(())
}
pub fn switch_player(player: Player) -> Player {



    let new_player = match player {
        Player::Blue => Player::Red,
        Player::Red => Player::Blue,
        Player::Neutral => player,
    };
    println!("switched Player");
    
    new_player
}
fn gravity_basic(grid: &Grid, column: i32, start_row: i32) -> Result<i32, GameError> {
    let mut row = 0;
    let mut occupation_counter = 0;

    for i in start_row..6 {
        println!("is column: {} i: {} row: {} free?", column, i, row );
        match &grid.grid[column as usize][i as usize].status {
    
    
            CellStatus::Occupied => {
                println!("occupied");
                if i < 1 {
                    return Err(GameError::ColumnFull);
                } else {
                    occupation_counter += 1;
                    row = i - occupation_counter;
                }
            },
            CellStatus::Free => {
                println!("free");
                row = i;
            },
        }
    }
    println!("final row value {}", row);
    Ok(row)
}

fn detect_win(grid: &Grid, ) {

    let down = (col - 0, row - 1);
    let up = (col - 0, row + 1);
    let left = (col - 1, row - 0);
    let right= (col + 1, row - 0);
    let down_left = (col - 1, row -1);
    let up_right = (col + 1, row +1);
    let down_right = (col + 1, row - 1);
    let up_left = (col - 1, row + 1);

    let surroundings = [down, up, left, right, down_left, up_right, down_right, up_left];

    for col in 1 .. 5 {
        for row in 1..4 {

        }

    }
    for direction in surroundings {
        if &grid.grid[direction.0 as usize][direction.1 as usize].player == player {

            if &grid.grid[direction.0 as usize][direction.1 as usize].player 





        }

    }
    
    {

            }

        }
    }
}

fn check_neighbors(grid: &Grid, player: Player) {

}