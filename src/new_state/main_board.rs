use crate::{
	consts::{MAIN_BOARD_COLS, MAIN_BOARD_ROWS},
	core::{
		board::{Board, Cell},
		tetromino_kind::TetrominoKind,
	},
};

pub struct MainBoard {
	cells: Vec<Cell>,
}

impl MainBoard {
	pub fn new() -> Self {
		Self {
			cells: Vec::with_capacity(MAIN_BOARD_COLS * MAIN_BOARD_ROWS),
		}
	}
}

impl Board for MainBoard {
	fn get_cell_kind(&self, x: usize, y: usize) -> Option<&TetrominoKind> {
		self.cells
			.iter()
			.find(|cell| cell.point.x == x as i8 && cell.point.y == y as i8)
			.map(|c| &c.kind)
	}
}
