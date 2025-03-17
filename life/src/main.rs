use game_logic::{change_cell_state, check_cell_state, manage_cell_state};
use gui::{setup_frame, show_text};
use macroquad::{input::{get_last_key_pressed, is_mouse_button_down, mouse_position, KeyCode}, time::get_time, window::{next_frame, Conf}};
use utils::calculate_cell_position_from_mouse;
mod gui;
mod game_logic;
mod utils;

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

/// Sets the configuration of the screen used.
/// 
/// # Returns
/// 
/// A struct `Conf` with the configuration needed
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut matrix: [[bool; COLS]; ROWS] = [[DEAD; COLS]; ROWS];
    let mut begin_life = false;
    // Calculation of cell dimensions so the whole screen is used.
    let cell_width = GRID_WIDTH / COLS as f32;
    let cell_height = GRID_HEIGHT / ROWS as f32;
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
