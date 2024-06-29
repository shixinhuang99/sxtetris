use std::collections::VecDeque;

use super::{point::Points, Tetromino, TetrominoType};

#[derive(Clone)]
pub struct BoardState {
	pub board: VecDeque<Vec<TetrominoType>>,
	pub rows: usize,
	pub cols: usize,
}

impl BoardState {
	pub fn new(rows: usize, cols: usize) -> Self {
		let board = VecDeque::from(vec![vec![TetrominoType::None; cols]; rows]);

		Self {
			board,
			rows,
			cols,
		}
	}

	pub fn reset(&mut self) {
		self.board = VecDeque::from(vec![
			vec![TetrominoType::None; self.cols];
			self.rows
		]);
	}

	pub fn get_cell(&self, x: usize, y: usize) -> &TetrominoType {
		&self.board[y][x]
	}

	pub fn update_area(&mut self, tm: &Tetromino) {
		for p in tm.points.usize_points() {
			self.board[p.1][p.0] = tm.tm_type;
		}
	}

	pub fn clear_area(&mut self, tm: &Tetromino) {
		for p in tm.points.usize_points() {
			self.board[p.1][p.0] = TetrominoType::None;
		}
	}

	pub fn clear_area_if<F>(&mut self, tm: &Tetromino, should_clear: F)
	where
		F: Fn(&TetrominoType) -> bool,
	{
		for p in tm.points.usize_points() {
			let tm_type = &mut self.board[p.1][p.0];
			if should_clear(tm_type) {
				*tm_type = TetrominoType::None;
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
			if ignore_points.iter().any(|other| p == other) {
				return false;
			}
			!self.board[p.1][p.0].is_none_or_ghost()
		})
	}

	pub fn is_collision(&self, points: &Points) -> bool {
		points
			.usize_points()
			.iter()
			.any(|p| !self.board[p.1][p.0].is_none_or_ghost())
	}

	pub fn clear_line(&mut self) -> u32 {
		let mut cnt = 0;

		self.board.retain(|line| {
			if line.iter().any(|tm_type| tm_type.is_none_or_ghost()) {
				return true;
			}
			cnt += 1;
			false
		});

		for _ in 0..cnt {
			self.board.push_front(vec![TetrominoType::None; self.cols]);
		}

		cnt
	}

	pub fn serialize(&self) -> String {
		let mut content = String::from("#board\n");

		for rows in &self.board {
			for tm_type in rows {
				content.push(char::from(*tm_type));
			}
		}
		content.push('\n');

		content
	}

	pub fn deserialize(&mut self, source: &str) {
		let mut y = 0;
		let mut x = 0;

		for ch in source.chars() {
			self.board[y][x] = TetrominoType::from(ch);
			x += 1;
			if x > 9 {
				x = 0;
				y += 1;
			}
		}
	}
}
