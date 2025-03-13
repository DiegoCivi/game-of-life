use std::{cell, thread::sleep, time::Duration};

const N: usize = 5;
const OFFSETS: [i8; 3] = [-1, 0, 1];
const DEAD: u8 = 0;
const ALIVE: u8 = 1;

fn is_position_valid(row_i: i8, col_i: i8) -> bool {
    (row_i >= 0 || row_i < N as i8) || (col_i >= 0 || col_i < N as i8)
}

fn is_different_cell(row_i_1: i8, col_i_1: i8, row_i_2: i8, col_i_2: i8) -> bool {
    row_i_1 != row_i_2 && col_i_1 != col_i_2
}

fn manage_cell_state(cells: Vec<(usize, usize)>, state: u8, matrix: &mut [[u8; N]; N]) {
    for (row_i, col_i) in cells {
        matrix[row_i][col_i] = state;
    }
}

fn check_cell_alive_neighbours(col_i: usize, row_i: usize, matrix: [[u8; N]; N]) -> i32 {
    let mut alive_neighbours = 0;
    // This parsing is done here so we don't do it in every loop 
    let parsed_row_i = row_i as i8;
    let parsed_col_i = col_i as i8;

    // Check all neighbours. This means we have to check on 3 rows and 3 columns.
    for _ in 0..3 {
        for offset in OFFSETS {
            let row_to_check = parsed_row_i + offset;
            let col_to_check = parsed_col_i + offset;

            if !is_position_valid(row_to_check, col_to_check) 
            && is_different_cell(parsed_row_i, parsed_col_i, row_to_check, col_to_check) {
                continue;
            };

            let cell_to_check = matrix[row_to_check as usize][col_to_check as usize];

            if cell_to_check == 1 {
                alive_neighbours += 1;
            }
        }
    }

    alive_neighbours
}

fn main() {
    let mut matrix: [[u8; N]; N] = [[DEAD; N]; N];
    
    
    loop {    
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
        
        manage_cell_state(cells_to_kill, DEAD, &mut matrix);
        manage_cell_state(cells_to_revive, ALIVE, &mut matrix);

        // for (row_i, col_i) in cells_to_kill {
        //     matrix[row_i][col_i] = DEAD;
        // }
        // for (row_i, col_i) in cells_to_revive {
        //     matrix[row_i][col_i] = ALIVE;
        // }

        sleep(Duration::from_secs(5));
    }

}
