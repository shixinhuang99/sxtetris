use super::{position::Position, TetrominoKind};

pub struct BoardState {
	pub board: Vec<Vec<TetrominoKind>>,
	pub rows: usize,
	pub cols: usize,
}

impl BoardState {
	pub fn new(rows: usize, cols: usize) -> Self {
		#[cfg(feature = "_dev")]
		log::trace!("rows: {}, cols: {}", rows, cols,);

		Self {
			board: vec![vec![TetrominoKind::None; cols]; rows],
			rows,
			cols,
		}
	}

	pub fn get_cell(&self, x: usize, y: usize) -> &TetrominoKind {
		&self.board[y][x]
	}

	pub fn update_area(&mut self, position: &Position, value: TetrominoKind) {
		for p in position.points.iter() {
			self.board[p.1][p.0] = value;
		}
	}

	pub fn clear_area(&mut self, position: &Position) {
		self.update_area(position, TetrominoKind::None);
	}

	pub fn clear_area_if<F>(&mut self, position: &Position, should_update: F)
	where
		F: Fn(&TetrominoKind) -> bool,
	{
		for p in position.points.iter() {
			if should_update(&self.board[p.1][p.0]) {
				self.board[p.1][p.0] = TetrominoKind::None;
			}
		}
	}
}
