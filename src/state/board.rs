use super::{point::Points, Tetromino, TetrominoKind};

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

	pub fn get_cell(&self, x: usize, y: usize) -> &TetrominoKind {
		&self.board[y][x]
	}

	pub fn update_area(&mut self, tm: &Tetromino) {
		for p in tm.points.usize_points() {
			self.board[p.1][p.0] = tm.kind;
		}
	}

	pub fn clear_area(&mut self, tm: &Tetromino) {
		for p in tm.points.usize_points() {
			self.board[p.1][p.0] = TetrominoKind::None;
		}
	}

	pub fn clear_area_if<F>(&mut self, tm: &Tetromino, should_update: F)
	where
		F: Fn(&TetrominoKind) -> bool,
	{
		for p in tm.points.usize_points() {
			let kind = &mut self.board[p.1][p.0];
			if should_update(kind) {
				*kind = TetrominoKind::None;
			}
		}
	}

	pub fn is_collision(&self, points: &Points, ignore: &Points) -> bool {
		let ignore_points = ignore.usize_points();

		points.usize_points().iter().any(|p| {
			if ignore_points
				.iter()
				.any(|other| p.0 == other.0 && p.1 == other.1)
			{
				return false;
			}
			!matches!(
				self.board[p.1][p.0],
				TetrominoKind::None | TetrominoKind::Ghost
			)
		})
	}
}
