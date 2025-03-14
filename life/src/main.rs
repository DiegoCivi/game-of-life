use macroquad::{color::{BLUE, RED, WHITE}, shapes::draw_rectangle, window::{clear_background, next_frame, screen_height, screen_width, Conf}};

const N: usize = 4;
const M: usize = 4;
const OFFSETS: [i8; 3] = [-1, 0, 1];
const DEAD: u8 = 0;
const ALIVE: u8 = 1;

fn is_position_valid(row_i: i8, col_i: i8) -> bool {
    (row_i >= 0 && row_i < N as i8) && (col_i >= 0 && col_i < N as i8)
}

fn is_different_cell(row_i_1: i8, col_i_1: i8, row_i_2: i8, col_i_2: i8) -> bool {
    row_i_1 != row_i_2 || col_i_1 != col_i_2
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
    for row_offset in OFFSETS {
        let row_to_check = parsed_row_i + row_offset;
        for col_offset in OFFSETS {
            let col_to_check = parsed_col_i + col_offset;
            
            if !is_position_valid(row_to_check, col_to_check) 
            || !is_different_cell(parsed_row_i, parsed_col_i, row_to_check, col_to_check) {
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

fn draw_cells_grid(cell_witdh: f32, cell_height: f32) {
    for i in 0..M {
        let y = cell_height * i as f32;
        for j in 0..N {
            let x = cell_witdh * j as f32;
            let color = if (i + j) % 2 != 0 { RED } else { BLUE };
            draw_rectangle(x, y, cell_witdh, cell_height, color);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut matrix: [[u8; N]; N] = [
        [0,1,0,0],
        [0,1,0,0],
        [0,1,0,0],
        [0,0,0,0],
        ];

    // let game_size = screen_width().min(screen_height());    
    let cell_width = screen_width() / N as f32;
    let cell_height = screen_height() / M as f32;
    loop {
        clear_background(WHITE);
        draw_cells_grid(cell_width, cell_height);

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

        next_frame().await;
    }

}
