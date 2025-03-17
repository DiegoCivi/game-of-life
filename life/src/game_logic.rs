use crate::{utils::{is_different_cell, is_position_valid}, ALIVE, COLS, DEAD, OFFSETS, ROWS};

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
pub fn check_cell_alive_neighbours(col_i: usize, row_i: usize, matrix: &[[bool; COLS]; ROWS]) -> i32 {
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

/// Changes the state of specific cells in the matrix.
/// 
/// # Arguments
/// 
/// - `cells`: A Vec which contains tuples that represent the cells in the amtrix to modify.
/// - `state`: Represents wether the cell is alive or dead.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
pub fn manage_cell_state(cells: Vec<(usize, usize)>, state: bool, matrix: &mut [[bool; COLS]; ROWS]) {
    for (row_i, col_i) in cells {
        matrix[row_i][col_i] = state;
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
pub fn check_cell_state(matrix: &[[bool; COLS]; ROWS]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
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

/// Changes the cell state to the opposite of its current one
/// 
/// # Arguments
/// 
/// - `row_i`: Represents the row in the matrix of the cell to change.
/// - `col_i`: Represents the col in the matrix of the cell to change.
/// - `matrix`: An array with arrays that represent the matrix which contains every cell.
pub fn change_cell_state(row_i: usize, col_i: usize, matrix: &mut [[bool; COLS]; ROWS]) {
    let curr_state = matrix[row_i][col_i];
    let new_state = if curr_state == ALIVE { DEAD } else { ALIVE };
    matrix[row_i][col_i] = new_state
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

