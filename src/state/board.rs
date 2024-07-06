use std::collections::VecDeque;

use super::{confetti::ConfettiState, point::Points, Tetromino, TetrominoType};

pub struct BoardState {
	pub board: VecDeque<Vec<TetrominoType>>,
	pub rows: usize,
	pub cols: usize,
	cleared_rows: Vec<usize>,
	col_cursor: usize,
	pub status: BoardStatus,
	pub confetti: ConfettiState,
	pub confetti_enable: bool,
}

#[derive(PartialEq)]
pub enum BoardStatus {
	None,
	Pending,
	Done,
}

impl BoardState {
	pub fn new(rows: usize, cols: usize) -> Self {
		let board = VecDeque::from(vec![vec![TetrominoType::None; cols]; rows]);

		Self {
			board,
			rows,
			cols,
			cleared_rows: Vec::new(),
			col_cursor: 0,
			status: BoardStatus::None,
			confetti: ConfettiState::new(),
			confetti_enable: true,
		}
	}

	pub fn reset(&mut self) {
		self.board = VecDeque::from(vec![
			vec![TetrominoType::None; self.cols];
			self.rows
		]);
		self.cleared_rows.clear();
		self.col_cursor = 0;
		self.status = BoardStatus::None;
		self.confetti.reset();
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

	pub fn check_need_cleared_rows(&mut self) -> usize {
		for (i, row) in self.board.iter().enumerate() {
			if row.iter().any(|tm_type| tm_type.is_none_or_ghost()) {
				continue;
			}
			self.cleared_rows.push(i);
		}
		if !self.cleared_rows.is_empty() {
			self.status = BoardStatus::Pending;
		}
		self.cleared_rows.len()
	}

	fn clear_col(&mut self) {
		for (y, row) in self.board.iter_mut().enumerate() {
			if !self.cleared_rows.contains(&y) {
				continue;
			}
			for (x, cell) in row.iter_mut().enumerate() {
				if x == self.col_cursor {
					*cell = TetrominoType::None;
					if self.confetti_enable {
						self.confetti.push_points(x, y);
					}
				}
			}
		}
	}

	fn push_new_rows(&mut self) {
		for row in &self.cleared_rows {
			self.board.remove(*row);
			self.board.push_front(vec![TetrominoType::None; self.cols]);
		}
		self.cleared_rows.clear();
	}

	pub fn update_clear_rows_progress(&mut self) {
		self.confetti.update_particles();
		if self.status != BoardStatus::Pending {
			return;
		}
		self.clear_col();
		if self.col_cursor >= self.cols {
			self.col_cursor = 0;
			self.status = BoardStatus::Done;
		} else {
			self.col_cursor += 1;
		}
		if self.status == BoardStatus::Done {
			self.push_new_rows();
		}
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
