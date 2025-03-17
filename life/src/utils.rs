use crate::{COLS, GRID_HEIGHT, ROWS};

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
pub fn is_position_valid(row_i: i8, col_i: i8) -> bool {
    (row_i >= 0 && row_i < ROWS as i8) && (col_i >= 0 && col_i < COLS as i8)
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
pub fn is_different_cell(row_i_1: i8, col_i_1: i8, row_i_2: i8, col_i_2: i8) -> bool {
    row_i_1 != row_i_2 || col_i_1 != col_i_2
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
pub fn calculate_cell_position_from_mouse(mouse_x: f32, mouse_y: f32, cell_witdh: f32, cell_height: f32) -> Option<(usize, usize)> {
    // Since the height of the grid is not the same as the one 
    // of the screen we have to take this into account when we check the Y axis (the rows).
    if mouse_y > GRID_HEIGHT {
        return None;
    }
    let row_i = mouse_y / cell_height;
    let col_i = mouse_x / cell_witdh;
    Some((row_i as usize, col_i as usize))
}
