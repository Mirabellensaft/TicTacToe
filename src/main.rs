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
use game_lib::canvas;
use game_lib::types::GameEvent;
use game_lib::types::Player;

// this is main
fn main() {
    // canvas width in pixels
    let canvas_width = 300_u32;
    let canvas_height = 300_u32;

    // number of rows and colums
    let columns = 3_u32;
    let rows = 3_u32;

    let cell_width = canvas_width / columns;

    let (mut canvas, mut events) = canvas::init(canvas_width, canvas_height);
    let mut grid = canvas::grid_init(columns, rows);

    let mut player = Player::Red;

    thread::spawn(move || {});
    'game: loop {
        let mouse_status = MouseState::new(&events);
        for event in events.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let row = mouse_status.y() / 100;
                    let column = mouse_status.x() / 100;
                    // println!("mouse row: {}, col: {}", mouse_status.y(), mouse_status.x());
                    // println!("row: {}, col: {}", row, column);
                    let result =
                        game_lib::game::update_grid_with_new_mark(&mut grid, column, row, &player);

                    match result {
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

                    player = game_lib::game::switch_player(player);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }

        canvas::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);
        thread::sleep(time::Duration::from_millis(500));
    }
}
