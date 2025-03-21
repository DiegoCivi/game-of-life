use macroquad::{color::{BLACK, LIGHTGRAY, RED, WHITE}, input::{get_last_key_pressed, is_mouse_button_down, mouse_position, KeyCode}, shapes::{draw_line, draw_rectangle}, text::draw_text, time::get_time, window::{clear_background, next_frame, Conf}};

const DIMENSION: usize = 3;
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

/// Abstraction of the Matrix where every cell lives.
struct Matrix {
    inner: Vec<Vec<bool>>
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        let mut inner = Vec::new();
        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                row.push(false);
            }
            inner.push(row);
        }
        Self { inner }
    }
    
    fn new_from_vector(vec_matrix: Vec<Vec<bool>>) -> Self {
        Self { inner: vec_matrix }
    }
}

/// Abstraction of a point in the grid.
/// It has the form of (row, column)
#[derive(PartialEq, Eq)]
struct Point {
    col: usize,
    row: usize,
}

impl Point {
    /// Verifies if the position represented by the point is inside
    /// the matrix. 
    /// 
    /// # Returns
    /// 
    /// A `bool` indicating if this is true or not.
    fn is_inside_grid(&self) -> bool {
        self.row < DIMENSION && self.col < DIMENSION
    }

    /// Gets all the possible 8 neighbours of the point
    /// 
    /// ### Returns
    /// 
    /// A vector of points that represent all the neighbours
    fn get_neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for row_offset in OFFSETS {
            let row_to_check = self.row as i8 + row_offset;
            for col_offset in OFFSETS {
                let col_to_check = self.col as i8 + col_offset;
                // Create the point we want to use
                let neighbour_point = Point{ row: row_to_check as usize, col: col_to_check as usize };
    
                if !neighbour_point.is_inside_grid() 
                || neighbour_point == *self {
                    continue;
                };
                
                neighbours.push(neighbour_point);
            }
        }
        neighbours
    }
}

/// Changes the state of specific cells in the matrix.
/// 
/// # Arguments
/// 
/// - `cells`: A Vec which contains tuples that represent the cells in the amtrix to modify.
/// - `state`: Represents wether the cell is alive or dead.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn manage_cell_state(points: &[Point], state: bool, matrix: &mut [[bool; DIMENSION]; DIMENSION]) {
    for point in points {
        matrix[point.row][point.col] = state;
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
fn check_cell_alive_neighbours(point: &Point, matrix: &[[bool; DIMENSION]; DIMENSION]) -> i32 {
    let mut alive_neighbours = 0;
    let neighbours = point.get_neighbours();
    for p in neighbours {
        if matrix[p.row][p.col] == ALIVE {
            alive_neighbours += 1;
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
fn draw_cells_grid(cell_witdh: f32, cell_height: f32, matrix: &[[bool; DIMENSION]; DIMENSION]) {
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
fn check_cell_state(matrix: &[[bool; DIMENSION]; DIMENSION]) -> (Vec<Point>, Vec<Point>) {
    let mut cells_to_revive: Vec<Point> = Vec::new();
    let mut cells_to_kill: Vec<Point> = Vec::new();

    for (row_i, row) in matrix.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            let point = Point{ row: row_i, col: col_i };
            let alive_neighbours = check_cell_alive_neighbours(&point, matrix);

            if *cell == DEAD && alive_neighbours == 3 {
                cells_to_revive.push(point);
            } else if *cell == ALIVE && (alive_neighbours < 2 || alive_neighbours > 3) {
                cells_to_kill.push(point);
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
fn change_cell_state(row_i: usize, col_i: usize, matrix: &mut [[bool; DIMENSION]; DIMENSION]) {
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
fn setup_frame(cell_width: f32, cell_height: f32, matrix: &[[bool; DIMENSION]; DIMENSION]) {
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

    for i in 0..DIMENSION {
        // Vertical line. It only moves on the X axis by a step of cell_width.
        x1 = cell_width * i as f32;
        y1 = 0.;
        x2 = x1;
        y2 = GRID_HEIGHT;
        draw_line(x1, y1, x2, y2, 2., LIGHTGRAY);
    }
    
    for i in 0..DIMENSION {
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
    let mut matrix = Matrix::new(5, 5);
    let mut begin_life = false;
    // Calculation of cell dimensions so the whole screen is used.
    let cell_width = GRID_WIDTH / DIMENSION as f32;
    let cell_height = GRID_HEIGHT / DIMENSION as f32;
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
                manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
                manage_cell_state(&cells_to_revive, ALIVE, &mut matrix);
            }

        }
        // Catches whenever the user presses the 'Enter' to change the game state
        if let Some(key_pressed) = get_last_key_pressed() {
            match key_pressed {
                KeyCode::Enter => begin_life = !begin_life,
                _=> {}
            }
        }
        // We show the text here at the end because for some reason if we put it in setup_frame()
        // it will not appear on the screen.
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

        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 

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
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_one_iteration_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
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

        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
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

        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
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
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, matrix_pattern_1);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, matrix_pattern_2);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
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
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
               
        assert_eq!(matrix, result_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
            
        assert_eq!(matrix, result_matrix);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        manage_cell_state(&cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(&cells_to_revive, ALIVE, &mut matrix); 
            
        assert_eq!(matrix, result_matrix);
    }

}
