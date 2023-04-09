// Dependencies go here
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{event::Event, mouse::MouseState};
use std::{thread, time};

mod lib;
use lib::types::Player;

use crate::lib::types::GameEvent;
// this is main
fn main() {
    let canvas_width = 300_u32;
    let canvas_height = 300_u32;

    let columns = 3_u32;
    let rows = 3_u32;

    let cell_width = canvas_width / columns;

    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let mut grid = lib::grid_init(columns, rows);

    let mut player = Player::Red;

    thread::spawn(move || {
        // some work here
    });
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
                    println!("mouse row: {}, col: {}", mouse_status.y(), mouse_status.x());
                    println!("row: {}, col: {}", row, column);
                    let result =
                        lib::game::update_grid_with_new_coin(&mut grid, column, row, &player);

                    match result {
                        Ok(()) => println!("Nothing to see here"),
                        Err(GameEvent::FieldOccupied) => {
                            println!("Field is occupied, please try another one.")
                        }
                        Err(GameEvent::GameTied) => {
                            println!("Game is tied.");
                            thread::sleep(time::Duration::from_millis(800));
                            lib::clear_grid(&mut grid);
                        }

                        Err(GameEvent::GameWon) => {
                            println!("Last player won!");
                            thread::sleep(time::Duration::from_millis(800));
                            lib::clear_grid(&mut grid);
                        }
                    }

                    player = lib::game::switch_player(player);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }

        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);
        thread::sleep(time::Duration::from_millis(500));
    }
}
