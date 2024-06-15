pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const BOARD_VISIBLE_Y_LEN: usize = 16;

pub const BOARD_X_LEN: usize = 10;

/// with buffer zone
pub const BOARD_Y_LEN: usize = BOARD_VISIBLE_Y_LEN * 2;

pub const PREVIEW_BOARD_Y_LEN: usize = 2;

pub const PREVIEW_BOARD_X_LEN: usize = 8;

pub const MIN_CELL_WIDTH: u16 = 5;

pub const MIN_CELL_HEIGHT: u16 = 3;
