use macroquad::{color::{BLACK, LIGHTGRAY, RED, WHITE}, shapes::{draw_line, draw_rectangle}, text::draw_text, window::clear_background};

use crate::{ALIVE, COLS, ROWS, WINDOW_HEIGHT};

const GRID_HEIGHT: f32 = WINDOW_HEIGHT as f32 - BUTTON_TAB_HEIGHT;
const GRID_WIDTH: f32 = 800.;
const BUTTON_TAB_HEIGHT: f32 = 100.;
const INSTRUCTIONS_TEXT: &str = "To start or stop press 'ENTER'";
const INSTRUCTIONS_TEXT_X: f32 = 200.;
const INSTRUCTIONS_TEXT_Y: f32 = 650.;
const INSTRUCTIONS_TEXT_SIZE: f32 = 30.;



/// Necessary setup for each frame, containing the drawing of the grid.
/// 
/// # Arguments
/// 
/// - `cell_width`: Width size of a single matrix cell.
/// - `cell_height`: Height size of a single matrix cell.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
pub fn setup_frame(cell_width: f32, cell_height: f32, matrix: &[[bool; COLS]; ROWS]) {
    clear_background(RED);
    draw_cells_grid(cell_width, cell_height, &matrix);
    draw_grid_lines(cell_width, cell_height);
}

/// Simple text drawing with the instructions for the user
pub fn show_text() {
    draw_text(INSTRUCTIONS_TEXT, INSTRUCTIONS_TEXT_X, INSTRUCTIONS_TEXT_Y, INSTRUCTIONS_TEXT_SIZE, BLACK);
}

/// Draws the lines of the grid.
/// 
/// # Arguments
/// 
/// - `cell_width`: Width size of a single matrix cell.
/// - `cell_height`: Height size of a single matrix cell.
fn draw_grid_lines(cell_width: f32, cell_height: f32) {
    let mut x1: f32;
    let mut y1: f32;
    let mut x2: f32;
    let mut y2: f32;

    for i in 0..ROWS {
        // Vertical line. It only moves on the X axis by a step of cell_width.
        x1 = cell_width * i as f32;
        y1 = 0.;
        x2 = x1;
        y2 = GRID_HEIGHT;
        draw_line(x1, y1, x2, y2, 2., LIGHTGRAY);
    }
    
    for i in 0..COLS {
        // Horizontal line. It only moves on the Y axis by a step of cell_height.
        x1 = 0.;
        y1 = cell_height * i as f32;
        x2 = GRID_WIDTH;
        y2 = y1;
        draw_line(x1, y1, x2, y2, 2., LIGHTGRAY);
    }
}

/// Draws a rectangle for each cell in the matrix, forming the necessary grid on the screen
/// 
/// # Arguments
/// 
/// - `cell_witdh`: The width of a single cell of the matrix.
/// - `cell_height`: The height of a single cell of the matrix.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn draw_cells_grid(cell_witdh: f32, cell_height: f32, matrix: &[[bool; COLS]; ROWS]) {
    for (row_i, row) in matrix.iter().enumerate() {
        let y = cell_height * row_i as f32;
        for (col_i, cell) in row.iter().enumerate() {
            let x = cell_witdh * col_i as f32;
            let color = if *cell == ALIVE { BLACK } else { WHITE };
            draw_rectangle(x, y, cell_witdh, cell_height, color);
        }
    }
}
