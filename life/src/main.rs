use macroquad::{color::{BLACK, LIGHTGRAY, RED, WHITE}, input::{get_last_key_pressed, is_mouse_button_down, mouse_position, KeyCode}, shapes::{draw_line, draw_rectangle}, text::draw_text, time::get_time, window::{clear_background, next_frame, Conf}};

const ROWS: usize = 3;
const COLS: usize = 3;
const OFFSETS: [i8; 3] = [-1, 0, 1];
const DEAD: bool = false;
const ALIVE: bool = true;
const FRAME_DELAY: f64 = 0.3;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_WIDTH: i32 = 800;
const GRID_HEIGHT: f32 = WINDOW_HEIGHT as f32 - BUTTON_TAB_HEIGHT;
const GRID_WIDTH: f32 = 800.;
const BUTTON_TAB_HEIGHT: f32 = 100.;
const INSTRUCTIONS_TEXT: &str = "To start or stop press 'ENTER'";
const INSTRUCTIONS_TEXT_X: f32 = 200.;
const INSTRUCTIONS_TEXT_Y: f32 = 650.;
const INSTRUCTIONS_TEXT_SIZE: f32 = 30.;


/// Verifies if the position represented by the point ('row_i', 'col_i') is inside
/// the matrix. 
/// 
/// # Arguments
/// 
/// - `row_i`: Represents the row in the matrix of the point.
/// - `col_i`: Represents the column in the matrix of the point.
/// 
/// # Returns
/// 
/// A `bool` indicating if this is true or not.
fn is_position_valid(row_i: i8, col_i: i8) -> bool {
    (row_i >= 0 && row_i < ROWS as i8) && (col_i >= 0 && col_i < ROWS as i8)
}

/// Verifies if 2 points on the matrix are the same point. 
/// 
/// # Arguments
/// 
/// - `row_i_1`: Represents the row in the matrix of point 1.
/// - `col_i_1`: Represents the column in the matrix of point 1.
/// - `row_i_2`: Represents the row in the matrix of point 2.
/// - `col_i_2`: Represents the column in the matrix of point 2.
/// 
/// # Returns
/// 
/// A `bool` indicating if this is true or not.
fn is_different_cell(row_i_1: i8, col_i_1: i8, row_i_2: i8, col_i_2: i8) -> bool {
    row_i_1 != row_i_2 || col_i_1 != col_i_2
}

/// Changes the state of specific cells in the matrix.
/// 
/// # Arguments
/// 
/// - `cells`: A Vec which contains tuples that represent the cells in the amtrix to modify.
/// - `state`: Represents wether the cell is alive or dead.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn manage_cell_state(cells: Vec<(usize, usize)>, state: bool, matrix: &mut [[bool; ROWS]; ROWS]) {
    for (row_i, col_i) in cells {
        matrix[row_i][col_i] = state;
    }
}

/// Checks the eight neighbours of a cell and counts how many of them are alive.
/// 
/// # Arguments
/// 
/// - `col_i_1`: Represents the column in the matrix of the cell.
/// - `row_i_1`: Represents the row in the matrix of the cell.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
/// 
/// # Returns
/// 
/// An `i32` that represents the number of alive neighbours.
fn check_cell_alive_neighbours(col_i: usize, row_i: usize, matrix: &[[bool; ROWS]; ROWS]) -> i32 {
    let mut alive_neighbours = 0;
    // This parsing is done here so we don't do it in every loop 
    let parsed_row_i = row_i as i8;
    let parsed_col_i = col_i as i8;

    // Check all neighbours. This means we have to check on 3 rows and 3 columns.
    for row_offset in OFFSETS {
        let row_to_check = parsed_row_i + row_offset;
        for col_offset in OFFSETS {
            let col_to_check = parsed_col_i + col_offset;
            
            if !is_position_valid(row_to_check, col_to_check) 
            || !is_different_cell(parsed_row_i, parsed_col_i, row_to_check, col_to_check) {
                continue;
            };

            let cell_to_check = matrix[row_to_check as usize][col_to_check as usize];

            if cell_to_check == ALIVE {
                alive_neighbours += 1;
            }
        }
    }

    alive_neighbours
}

/// Draws a rectangle for each cell in the matrix, forming the necessary grid on the screen
/// 
/// # Arguments
/// 
/// - `cell_witdh`: The width of a single cell of the matrix.
/// - `cell_height`: The height of a single cell of the matrix.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn draw_cells_grid(cell_witdh: f32, cell_height: f32, matrix: &[[bool; ROWS]; ROWS]) {
    for (row_i, row) in matrix.iter().enumerate() {
        let y = cell_height * row_i as f32;
        for (col_i, cell) in row.iter().enumerate() {
            let x = cell_witdh * col_i as f32;
            let color = if *cell == ALIVE { BLACK } else { WHITE };
            draw_rectangle(x, y, cell_witdh, cell_height, color);
        }
    }
}

/// Sets the configuration of the screen used.
/// 
/// # Returns
/// 
/// A struct `Conf` with the configuration needed
fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

/// Checks which cells should be revived and which cells should be killed.
/// 
/// # Arguments
/// 
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
/// 
/// # Returns
/// 
/// A tuple containing two `Vec<(usize, usize)>`. The first one represents all the cells
/// that should be brought back to life, while the second vector represents all the cells
/// that should be killed.
fn check_cell_state(matrix: &[[bool; ROWS]; ROWS]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut cells_to_revive: Vec<(usize, usize)> = Vec::new();
    let mut cells_to_kill: Vec<(usize, usize)> = Vec::new();

    for (row_i, row) in matrix.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            let alive_neighbours = check_cell_alive_neighbours(col_i, row_i, matrix);

            if *cell == DEAD && alive_neighbours == 3 {
                cells_to_revive.push((row_i, col_i));
            } else if *cell == ALIVE && (alive_neighbours < 2 || alive_neighbours > 3) {
                cells_to_kill.push((row_i, col_i));
            }
        }
    }

    (cells_to_revive, cells_to_kill)
}

/// Calculates which cell is the user choosing with its mouse. 
/// 
/// # Arguments
/// 
/// - `mouse_x`: Position on the X axis of the mouse.
/// - `mouse_y`: Position on the Y axis of the mouse.
/// - `cell_width`: Width size of a single matrix cell.
/// - `cell_height`: Height size of a single matrix cell.
/// 
/// # Returns
/// 
/// A tuple of usize in the format of (row, column) containing the mapped matrix cell. 
fn calculate_cell_position_from_mouse(mouse_x: f32, mouse_y: f32, cell_witdh: f32, cell_height: f32) -> Option<(usize, usize)> {
    // Since the height of the grid is not the same as the one 
    // of the screen we have to take this into account when we check the Y axis (the rows).
    if mouse_y > GRID_HEIGHT {
        return None;
    }
    let row_i = mouse_y / cell_height;
    let col_i = mouse_x / cell_witdh;
    Some((row_i as usize, col_i as usize))
}

/// Changes the cell state to the opposite of its current one
/// 
/// # Arguments
/// 
/// - `row_i`: Represents the row in the matrix of the cell to change.
/// - `col_i`: Represents the col in the matrix of the cell to change.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn change_cell_state(row_i: usize, col_i: usize, matrix: &mut [[bool; ROWS]; ROWS]) {
    let curr_state = matrix[row_i][col_i];
    let new_state = if curr_state == ALIVE { DEAD } else { ALIVE };
    matrix[row_i][col_i] = new_state
}

/// Simple text drawing with the instructions for the user
fn show_text() {
    draw_text(INSTRUCTIONS_TEXT, INSTRUCTIONS_TEXT_X, INSTRUCTIONS_TEXT_Y, INSTRUCTIONS_TEXT_SIZE, BLACK);
}

/// Necessary setup for each frame, containing the drawing of the grid.
/// 
/// # Arguments
/// 
/// - `cell_width`: Width size of a single matrix cell.
/// - `cell_height`: Height size of a single matrix cell.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn setup_frame(cell_width: f32, cell_height: f32, matrix: &[[bool; ROWS]; ROWS]) {
    clear_background(RED);
    draw_cells_grid(cell_width, cell_height, &matrix);
    draw_grid_lines(cell_width, cell_height);
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

        // Horizontal line. It only moves on the Y axis by a step of cell_height.
        x1 = 0.;
        y1 = cell_height * i as f32;
        x2 = GRID_WIDTH;
        y2 = y1;
        draw_line(x1, y1, x2, y2, 2., LIGHTGRAY);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut matrix: [[bool; ROWS]; ROWS] = [[DEAD; ROWS]; COLS];
    let mut begin_life = false;
    // Calculation of cell dimensions so the whole screen is used.
    let cell_width = GRID_WIDTH / ROWS as f32;
    let cell_height = GRID_HEIGHT / COLS as f32;
    // Used for updating each frame after a desired time
    let mut last_update = get_time();
    loop {
        setup_frame(cell_width, cell_height, &matrix);
        
        // After 2 seconds, the matrix is changed so it updates in the next frame
        if get_time() - last_update > FRAME_DELAY { 
            last_update = get_time();

            // If the life didn't began, it means the player is still choosing
            // which cell pattern to start with
            if !begin_life {
                if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    match calculate_cell_position_from_mouse(mouse_x, mouse_y, cell_width, cell_height) {
                        Some((row_i, col_i)) => change_cell_state(row_i, col_i, &mut matrix),
                        None => {},
                    }
                }
            } else { // If the life began, cells need to start dying and reviving
                let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
                manage_cell_state(cells_to_kill, DEAD, &mut matrix);
                manage_cell_state(cells_to_revive, ALIVE, &mut matrix);
            }

        }
        if let Some(key_pressed) = get_last_key_pressed() {
            match key_pressed {
                KeyCode::Enter => begin_life = !begin_life,
                _=> {}
            }
        }
        show_text();
        next_frame().await;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alone_cell_dies() {
        let mut matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ];
        let result_matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ];
        
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);

        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 

        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn two_neighbours_survives_then_dies() {
        let mut matrix: [[bool; 3]; 3] = [
            [false, false, true],
            [false, true, false],
            [true, false, false],
        ];
        let result_one_iteration_matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ];
        let result_two_iterations_matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ];
        
        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_one_iteration_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_two_iterations_matrix);

    }

    #[test]
    fn only_one_neighbour_both_die() {
        let mut matrix: [[bool; 3]; 3] = [
            [false, true, false],
            [false, true, false],
            [false, false, false],
        ];
        let result_matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ];
        
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);

        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn four_neighbours_cell_dies() {
        let mut matrix: [[bool; 3]; 3] = [
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ];
        let result_matrix: [[bool; 3]; 3] = [
            [false, true, false],
            [true, false, true],
            [false, true, false],
        ];
        
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);

        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn simple_repetitive_pattern() {
        let mut matrix: [[bool; 3]; 3] =[
            [false, true, false],
            [false, true, false],
            [false, true, false],
        ];
        let matrix_pattern_1: [[bool; 3]; 3] =[
            [false, false, false],
            [true, true, true],
            [false, false, false],
        ];
        let matrix_pattern_2: [[bool; 3]; 3] = [
            [false, true, false],
            [false, true, false],
            [false, true, false],
        ];
        
        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, matrix_pattern_1);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, matrix_pattern_2);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, matrix_pattern_1);
    }


    #[test]
    fn three_neighbours_cell_survives_forever() {
        let mut matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, true, true],
            [false, true, true],
        ];
        let result_matrix: [[bool; 3]; 3] = [
            [false, false, false],
            [false, true, true],
            [false, true, true],
        ];
    
        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
            
        assert_eq!(matrix, result_matrix);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix); 
            
        assert_eq!(matrix, result_matrix);
    }

}
