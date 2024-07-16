use crate::{
	consts::{NEXT_BOARD_COLS, NEXT_BOARD_ROWS},
	core::{tetromino_kind::TetrominoKind, Board},
};

pub struct NextBoard {
	cells: Vec<Vec<Option<TetrominoKind>>>,
}

impl NextBoard {
	pub fn new() -> Self {
		Self {
			cells: vec![vec![None; NEXT_BOARD_COLS]; NEXT_BOARD_ROWS],
		}
	}

	pub fn set_next(&mut self, next: TetrominoKind) {
		self.cells.clear();

		let mut position = next.init_position(0);
		position.update(|p| p.x += 3);

		for p in position.to_board_points() {
			self.cells[p.y][p.x] = Some(next);
		}
	}
}

impl Board for NextBoard {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind> {
		self.cells[y][x].as_ref()
	}
}
