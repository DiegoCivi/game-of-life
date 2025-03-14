use std::{thread::sleep, time::Duration};

const N: usize = 3;
const OFFSETS: [i8; 3] = [-1, 0, 1];
const DEAD: bool = false;
const ALIVE: bool = true;

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
    (row_i >= 0 && row_i < N as i8) && (col_i >= 0 && col_i < N as i8)
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
fn manage_cell_state(cells: Vec<(usize, usize)>, state: bool, matrix: &mut [[bool; N]; N]) {
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
fn check_cell_alive_neighbours(col_i: usize, row_i: usize, matrix: &[[bool; N]; N]) -> i32 {
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
fn check_cell_state(matrix: &[[bool; N]; N]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
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

fn main() {
    let mut matrix: [[bool; N]; N] = [
        [false,true,false],
        [false,true,false],
        [false,true,false],
    ];
    
    loop {
        let (cells_to_revive, cells_to_kill) = check_cell_state(&matrix);
        
        if cells_to_kill.is_empty() && cells_to_revive.is_empty() {
            break;
        }

        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix);

        // This sleep is to make the output more readeable
        sleep(Duration::from_secs(5));
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
