// this module contains all logic around the canvas and displaying playing field
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::game_lib::types::{Cell, CellColor, CellStatus, Grid, Player};

/// This function initializes the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Canvas", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

/// Creates a grid with ncells*ncells initialized with cell in a color
pub fn grid_init(nx_cells: u32, ny_cells: u32) -> Grid {
    let mut grid_vector = Vec::new();

    for col in 0..nx_cells {
        grid_vector.push(Vec::new());
        for _row in 0..ny_cells {
            grid_vector[col as usize].push(Cell {
                player: Player::Neutral,
                status: CellStatus::Free,
            });
        }
    }
    let grid = Grid { grid: grid_vector };

    grid
}

/// Converts row column values into xy pixels and draws rectangle in the specified color
pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    grid_data: &Grid,
    cell_width: &u32,
) {
    let cell_height = cell_width;

    let grid = &grid_data.grid;

    let x = cell_width * col;
    let y = cell_width * row;

    let cell_color = match &grid[col as usize][row as usize].player {
        Player::Blue => CellColor {
            red: 0_u8,
            green: 0_u8,
            blue: 125_u8,
        },
        Player::Red => CellColor {
            red: 125_u8,
            green: 0_u8,
            blue: 0_u8,
        },
        Player::Neutral => CellColor {
            red: 35_u8,
            green: 15_u8,
            blue: 13_u8,
        },
    };
    let drawing_color = Color::RGB(cell_color.red, cell_color.green, cell_color.blue);

    renderer.set_draw_color(drawing_color);
    let result = renderer.fill_rect(Rect::new(x as i32, y as i32, *cell_width, *cell_height));
    match result {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}

/// Displays the whole grid by repeatedly calling display_cell on every cell
pub fn display_frame(
    renderer: &mut Canvas<Window>,
    grid: &Grid,
    nx_cells: &u32,
    ny_cells: &u32,
    cell_width: &u32,
) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for column in 0..*nx_cells {
        for row in 0..*ny_cells {
            display_cell(renderer, row, column, &grid, &cell_width)
        }
    }
    renderer.present();
}

/// Clears the grid
pub fn clear_grid(grid: &mut Grid) {
    println!("clearing grid");

    let max_columns = &grid.grid.len();
    let max_rows = &grid.grid[0].len();

    for column in 0..*max_columns as i32 {
        for row in 0..*max_rows as i32 {
            grid.grid[column as usize][row as usize].player = Player::Neutral;
            grid.grid[column as usize][row as usize].status = CellStatus::Free;
        }
    }
}
