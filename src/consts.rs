pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const MATRIX_Y_VISIBLE_LEN: usize = 16;

pub const MATRIX_X_LEN: usize = 10;

/// with buffer zone
pub const MATRIX_Y_LEN: usize = MATRIX_Y_VISIBLE_LEN * 2;

pub const MATRIX_Y_VISIBLE_LEN_U16: u16 = MATRIX_Y_VISIBLE_LEN as u16;

pub const MATRIX_X_LEN_U16: u16 = MATRIX_X_LEN as u16;

pub const PREVIEW_MATRIX_Y_LEN: usize = 2;

pub const PREVIEW_MATRIX_X_LEN: usize = 8;

pub const MIN_CELL_WIDTH: u16 = 5;

pub const MIN_CELL_HEIGHT: u16 = 3;
