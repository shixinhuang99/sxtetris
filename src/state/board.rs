use super::TetrominoKind;

pub struct BoardState {
	pub board: Vec<Vec<TetrominoKind>>,
	pub rows: usize,
	pub cols: usize,
}

impl BoardState {
	pub fn new(rows: usize, cols: usize) -> Self {
		Self {
			board: vec![vec![TetrominoKind::None; cols]; rows],
			rows,
			cols,
		}
	}

	pub fn update_cell(&mut self, v: TetrominoKind, x: usize, y: usize) {
		#[cfg(feature = "_dev")]
		log::trace!(
			"cols: {}, rows: {}, x: {}, y: {}",
			self.cols,
			self.rows,
			x,
			y
		);

		self.board[y][x] = v;
	}

	pub fn get_cell(&self, x: usize, y: usize) -> &TetrominoKind {
		&self.board[y][x]
	}
}
