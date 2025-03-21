use std::ops::{Index, IndexMut};
use clap::Parser;
use macroquad::{color::{BLACK, LIGHTGRAY, RED, WHITE}, input::{get_last_key_pressed, is_mouse_button_down, mouse_position, KeyCode}, shapes::{draw_line, draw_rectangle}, text::draw_text, time::get_time, window::{clear_background, next_frame, Conf}};

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

/// Simple struct to receive cli params
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Dimension for the squared matrix
    #[arg(short, long, default_value_t = 1)]
    dimension: usize,
}

/// Abstraction of the Matrix where every cell lives.
#[derive(PartialEq, Debug)]
struct Matrix {
    inner: Vec<Vec<bool>>
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        let mut inner = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(false);
            }
            inner.push(row);
        }
        Self { inner }
    }
    
    fn new_from_vector(vec_matrix: Vec<Vec<bool>>) -> Self {
        Self { inner: vec_matrix }
    }

    fn dimension(&self) -> usize {
        self.inner.len()
    }

    /// Changes the cell state to the opposite of its current one
    /// 
    /// # Arguments
    /// 
    /// - `point`: Represents a specific point on the grid.
    fn change_cell_state(&mut self, point: Point) {
        let curr_state = self[point];
        let new_state = if curr_state == ALIVE { DEAD } else { ALIVE };
        self[point] = new_state
    }

    /// Checks the eight neighbours of a cell and counts how many of them are alive.
    /// 
    /// # Arguments
    /// 
    /// - `point`: A point representing a cell in the grid. 
    /// 
    /// # Returns
    /// 
    /// An `i32` that represents the number of alive neighbours.
    fn check_cell_alive_neighbours(&self, point: &Point) -> i32 {
        let mut alive_neighbours = 0;
        let neighbours = point.get_neighbours(self.dimension());
        for p in neighbours {
            if self[p] == ALIVE {
                alive_neighbours += 1;
            }
        }
        alive_neighbours
    }

    /// Checks which cells should be revived and which cells should be killed.
    /// 
    /// # Returns
    /// 
    /// A tuple containing two `Vec<Point>`. The first one represents all the cells
    /// that should be brought back to life, while the second vector represents all the cells
    /// that should be killed.
    fn check_cells_state(&self) -> (Vec<Point>, Vec<Point>) {
        let mut cells_to_revive: Vec<Point> = Vec::new();
        let mut cells_to_kill: Vec<Point> = Vec::new();
    
        for (row_i, row) in self.inner.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                let point = Point{ row: row_i, col: col_i };
                let alive_neighbours = self.check_cell_alive_neighbours(&point);
    
                if *cell == DEAD && alive_neighbours == 3 {
                    cells_to_revive.push(point);
                } else if *cell == ALIVE && (alive_neighbours < 2 || alive_neighbours > 3) {
                    cells_to_kill.push(point);
                }
            }
        }
    
        (cells_to_revive, cells_to_kill)     
    }

    fn kill_cells(&mut self, cells: &Vec<Point>) {
        self.manage_cells_state(cells, DEAD);
    }

    fn revive_cells(&mut self, cells: &Vec<Point>) {
        self.manage_cells_state(cells, ALIVE);
    }

    /// Changes the state of specific cells in the matrix.
    /// 
    /// # Arguments
    /// 
    /// - `cells`: A Vec which contains tuples that represent the cells in the matrix to modify.
    /// - `state`: Represents wether the cell is alive or dead.
    fn manage_cells_state(&mut self, cells: &Vec<Point>, state: bool) {
        for point in cells {
            self[*point] = state;
        }
    }

}

impl Index<Point> for Matrix {
    type Output = bool;

    fn index(&self, point: Point) -> &Self::Output {
        &self.inner[point.row][point.col]
    }
}

impl IndexMut<Point> for Matrix {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.inner[point.row][point.col]
    }
}

/// Abstraction of a point in the grid.
/// It has the form of (row, column)
#[derive(PartialEq, Eq, Copy, Clone)]
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
    fn is_inside_grid(&self, matrix_dimension: usize) -> bool {
        self.row < matrix_dimension && self.col < matrix_dimension
    }

    /// Gets all the possible 8 neighbours of the point
    /// 
    /// ### Returns
    /// 
    /// A vector of points that represent all the neighbours
    fn get_neighbours(&self, matrix_dimension: usize) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for row_offset in OFFSETS {
            let row_to_check = self.row as i8 + row_offset;
            for col_offset in OFFSETS {
                let col_to_check = self.col as i8 + col_offset;
                // Create the point we want to use
                let neighbour_point = Point{ row: row_to_check as usize, col: col_to_check as usize };
    
                if !neighbour_point.is_inside_grid(matrix_dimension) 
                || neighbour_point == *self {
                    continue;
                };
                
                neighbours.push(neighbour_point);
            }
        }
        neighbours
    }
}

/// Draws a rectangle for each cell in the matrix, forming the necessary grid on the screen
/// 
/// # Arguments
/// 
/// - `cell_witdh`: The width of a single cell of the matrix.
/// - `cell_height`: The height of a single cell of the matrix.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
fn draw_cells_grid(cell_witdh: f32, cell_height: f32, matrix: &Matrix) {
    for (row_i, row) in matrix.inner.iter().enumerate() {
        let y = cell_height * row_i as f32;
        for (col_i, cell_state) in row.iter().enumerate() {
            let x = cell_witdh * col_i as f32;
            let color = if *cell_state == ALIVE { BLACK } else { WHITE };
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

/// Calculates which cell is the user choosing with their mouse. 
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
fn calculate_cell_position_from_mouse(mouse_x: f32, mouse_y: f32, cell_witdh: f32, cell_height: f32) -> Option<Point> {
    // Since the height of the grid is not the same as the one 
    // of the screen we have to take this into account when we check the Y axis (the rows).
    if mouse_y > GRID_HEIGHT {
        return None;
    }
    let row = mouse_y / cell_height;
    let col = mouse_x / cell_witdh;
    Some(Point { col: col as usize, row: row as usize })
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
fn setup_frame(cell_width: f32, cell_height: f32, matrix: &Matrix) {
    clear_background(RED);
    draw_cells_grid(cell_width, cell_height, &matrix);
    draw_grid_lines(cell_width, cell_height, matrix.dimension());
}

/// Draws the lines of the grid.
/// 
/// # Arguments
/// 
/// - `cell_width`: Width size of a single matrix cell.
/// - `cell_height`: Height size of a single matrix cell.
fn draw_grid_lines(cell_width: f32, cell_height: f32, matrix_dimension: usize) {
    let mut x1: f32;
    let mut y1: f32;
    let mut x2: f32;
    let mut y2: f32;

    for i in 0..matrix_dimension {
        // Vertical line. It only moves on the X axis by a step of cell_width.
        x1 = cell_width * i as f32;
        y1 = 0.;
        x2 = x1;
        y2 = GRID_HEIGHT;
        draw_line(x1, y1, x2, y2, 2., LIGHTGRAY);
    }
    
    for i in 0..matrix_dimension {
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
    // Get the args from the cli
    let args = Args::parse();
    let mut matrix = Matrix::new(args.dimension, args.dimension);
    // This will let the game now if the user wants to stop the game to do some
    // changes or not
    let mut begin_life = false;
    // Calculation of cell dimensions so the whole screen is used.
    let cell_width = GRID_WIDTH / matrix.dimension() as f32;
    let cell_height = GRID_HEIGHT / matrix.dimension() as f32;
    // Used for updating each frame after a desired time
    let mut last_update = get_time();
    loop {
        setup_frame(cell_width, cell_height, &matrix);
        
        // After 2 seconds, the matrix is changed so it updates in the next frame
        if get_time() - last_update > FRAME_DELAY { 
            last_update = get_time();

            // If the life didn't begin, it means the player is still choosing
            // which cell pattern to start with
            if !begin_life {
                if is_mouse_button_down(macroquad::input::MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some(mouse_point) = calculate_cell_position_from_mouse(mouse_x, mouse_y, cell_width, cell_height) {
                        matrix.change_cell_state(mouse_point);
                    }
                }
            } else { // If the life began, cells need to start dying and reviving
                let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
                matrix.kill_cells(&cells_to_kill);
                matrix.revive_cells(&cells_to_revive);
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
        let matrix_vec = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let result_matrix_vec = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let result_matrix = Matrix::new_from_vector(result_matrix_vec);
        
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();

        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);

        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn two_neighbours_survives_then_dies() {
        let matrix_vec = vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, false, false],
        ];
        let result_one_iteration_matrix_vec = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let result_two_iterations_matrix_vec = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let result_one_iteration_matrix = Matrix::new_from_vector(result_one_iteration_matrix_vec);
        let result_two_iterations_matrix = Matrix::new_from_vector(result_two_iterations_matrix_vec);

        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, result_one_iteration_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, result_two_iterations_matrix);

    }

    #[test]
    fn only_one_neighbour_both_die() {
        let matrix_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let result_matrix_vec = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let result_matrix = Matrix::new_from_vector(result_matrix_vec);
        
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();

        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn four_neighbours_cell_dies() {
        let matrix_vec = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ]; 
        let result_matrix_vec = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let result_matrix = Matrix::new_from_vector(result_matrix_vec);

        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();

        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, result_matrix);
    }

    #[test]
    fn simple_repetitive_pattern() {
        let matrix_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let matrix_pattern_1_vec = vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ];
        let matrix_pattern_2_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let matrix_pattern_1 = Matrix::new_from_vector(matrix_pattern_1_vec);
        let matrix_pattern_2 = Matrix::new_from_vector(matrix_pattern_2_vec);
        
        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, matrix_pattern_1);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, matrix_pattern_2);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive); 
               
        assert_eq!(matrix, matrix_pattern_1);
    }


    #[test]
    fn three_neighbours_cell_survives_forever() {
        let matrix_vec = vec![
            vec![false, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ];
        let result_matrix_vec = vec![
            vec![false, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ];
        let mut matrix = Matrix::new_from_vector(matrix_vec);
        let result_matrix = Matrix::new_from_vector(result_matrix_vec);
    
        // Start of first iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
               
        assert_eq!(matrix, result_matrix);

        // Start of second iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
            
        assert_eq!(matrix, result_matrix);

        // Start of third iteration
        let (cells_to_revive, cells_to_kill) = matrix.check_cells_state();
        matrix.kill_cells(&cells_to_kill);
        matrix.revive_cells(&cells_to_revive);
            
        assert_eq!(matrix, result_matrix);
    }

}
