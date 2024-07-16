pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const APP_VER: &str = env!("CARGO_PKG_VERSION");

pub const BOARD_VISIBLE_ROWS: usize = 16;

pub const BOARD_COLS: usize = 10;

/// with buffer zone
pub const BOARD_ROWS: usize = BOARD_VISIBLE_ROWS * 2;

pub const PREVIEW_BOARD_ROWS: usize = 2;

pub const PREVIEW_BOARD_COLS: usize = 8;

pub const MIN_CELL_WIDTH: u16 = 5;

pub const MIN_CELL_HEIGHT: u16 = 3;

pub const FRAME_RATE_SECS: f32 = 0.033;

pub const MAIN_BOARD_COLS: usize = 10;
pub const MAIN_BOARD_BUFFER_ROWS: usize = 5;
pub const MAIN_BOARD_VISIBLE_ROWS: usize = 16;
pub const MAIN_BOARD_ROWS: usize =
	MAIN_BOARD_VISIBLE_ROWS + MAIN_BOARD_BUFFER_ROWS;

pub const NEXT_BOARD_COLS: usize = 8;
pub const NEXT_BOARD_ROWS: usize = 2;
