use super::{point::Points, TetrominoKind};

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

	pub fn update_area(&mut self, points: &Points, value: TetrominoKind) {
		for p in points.value.iter() {
			self.board[p.1 as usize][p.0 as usize] = value;
		}
	}

	pub fn clear_area(&mut self, points: &Points) {
		self.update_area(points, TetrominoKind::None);
	}

	pub fn clear_area_if<F>(&mut self, points: &Points, should_update: F)
	where
		F: Fn(&TetrominoKind) -> bool,
	{
		for p in points.value.iter() {
			let kind = &mut self.board[p.1 as usize][p.0 as usize];
			if should_update(kind) {
				*kind = TetrominoKind::None;
			}
		}
	}

	pub fn is_collision(&self, points: &Points, ignore: &Points) -> bool {
		points.value.iter().any(|p| {
			if ignore
				.value
				.iter()
				.any(|other| p.0 == other.0 && p.1 == other.1)
			{
				return false;
			}
			!matches!(
				self.board[p.1 as usize][p.0 as usize],
				TetrominoKind::None | TetrominoKind::Ghost
			)
		})
	}
}
