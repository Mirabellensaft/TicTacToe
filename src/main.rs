#![warn(
    array_into_iter,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    non_fmt_panics,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions
)]

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{event::Event, mouse::MouseState};
use std::{thread, time};

mod game_lib;
use game_lib::{
    canvas, game,
    types::{GameEvent, Player},
};

// Canvas and Playing Field Size

/// Canvas width in pixels
const CANVAS_WIDTH: u32 = 300;
/// Canvas height in pixels
const CANVAS_HEIGHT: u32 = 300;

/// Number of colums
const COLUMNS: u32 = 3;
/// Number of rows
const ROWS: u32 = 3;

const CELL_WIDTH: u32 = CANVAS_WIDTH / COLUMNS;

///# Main 
/// 
/// The main function contains the game loop. Most the of game's logic
/// can be found in the [game_lib::game] sub module. 
fn main() {
    let (mut canvas, mut pump_events) = canvas::init(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut grid = canvas::grid_init(COLUMNS, ROWS);

    // Starting Player
    let mut player = Player::Red;

    thread::spawn(move || {});
    'game: loop {
        let mouse_status = MouseState::new(&pump_events);
        for event in pump_events.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let mouse_position_row = mouse_status.y() / 100;
                    let mouse_position_column = mouse_status.x() / 100;

                    let game_event = game::update_grid_with_new_mark(
                        &mut grid,
                        mouse_position_column,
                        mouse_position_row,
                        &player,
                    );

                    match game_event {
                        Ok(()) => println!(""),
                        Err(GameEvent::FieldOccupied) => {
                            println!("Field is occupied, please try another one.")
                        }
                        Err(GameEvent::GameTied) => {
                            println!("Game is tied.");
                            thread::sleep(time::Duration::from_millis(800));
                            canvas::clear_grid(&mut grid);
                        }

                        Err(GameEvent::GameWon) => {
                            println!("Last player won!");
                            thread::sleep(time::Duration::from_millis(800));
                            canvas::clear_grid(&mut grid);
                        }
                    }

                    player = game::switch_player(player);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }

        canvas::display_frame(&mut canvas, &grid, &COLUMNS, &ROWS, &CELL_WIDTH);
        thread::sleep(time::Duration::from_millis(500));
    }
}
