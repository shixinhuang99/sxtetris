use crate::{
	common::{Board, TetrominoKind},
	consts::{NEXT_BOARD_COLS, NEXT_BOARD_ROWS},
};

pub struct NextBoard {
	cells: Vec<Vec<Option<TetrominoKind>>>,
	pub current: TetrominoKind,
}

impl NextBoard {
	pub fn new() -> Self {
		Self {
			cells: vec![vec![None; NEXT_BOARD_COLS]; NEXT_BOARD_ROWS],
			current: TetrominoKind::default(),
		}
	}

	pub fn set_next(&mut self, kind: TetrominoKind) {
		self.current = kind;
		self.cells.clear();

		let mut position = kind.init_position(0);
		position.update(|p| p.x += 3);

		for p in position.to_usize_points() {
			self.cells[p.y][p.x] = Some(kind);
		}
	}
}

impl Board for NextBoard {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind> {
		self.cells[y][x].as_ref()
	}
}
