use std::collections::VecDeque;

use super::{point::Points, Tetromino, TetrominoKind};

#[derive(Clone)]
pub struct BoardState {
	pub board: VecDeque<Vec<TetrominoKind>>,
	pub rows: usize,
	pub cols: usize,
}

impl BoardState {
	pub fn new(rows: usize, cols: usize) -> Self {
		let board = VecDeque::from(vec![vec![TetrominoKind::None; cols]; rows]);

		#[cfg(feature = "_dev")]
		{
			log::trace!("rows: {}, cols: {}", rows, cols);
			log::trace!(
				"board rows: {}, board cols: {}",
				board.len(),
				board[0].len()
			);
		}

		Self {
			board,
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

	pub fn is_collision_with_ignore(
		&self,
		points: &Points,
		ignore: &Points,
	) -> bool {
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

	pub fn is_collision(&self, points: &Points) -> bool {
		points.usize_points().iter().any(|p| {
			!matches!(
				self.board[p.1][p.0],
				TetrominoKind::None | TetrominoKind::Ghost
			)
		})
	}

	pub fn check_and_clear_line(&mut self) -> u32 {
		let mut cnt = 0;

		self.board.retain(|line| {
			if line.iter().any(|tm_kind| {
				matches!(tm_kind, TetrominoKind::None | TetrominoKind::Ghost)
			}) {
				return true;
			}
			cnt += 1;
			false
		});

		for _ in 0..cnt {
			self.board.push_front(vec![TetrominoKind::None; self.cols]);
		}

		cnt
	}

	pub fn stringify(&self) -> String {
		let mut content = String::from("#board\n");

		for rows in &self.board {
			for kind in rows {
				content.push(char::from(*kind));
			}
		}
		content.push('\n');

		content
	}

	pub fn read_save(&mut self, source: String) {
		let mut y = 0;
		let mut x = 0;

		for ch in source.chars() {
			self.board[y][x] = TetrominoKind::from(ch);
			x += 1;
			if x > 9 {
				x = 0;
				y += 1;
			}
		}
	}
}
