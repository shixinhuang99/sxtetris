use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::tetromino::Tetromino;
use crate::{
	common::{Board, Position, TetrominoKind},
	consts::{MAIN_BOARD_COLS, MAIN_BOARD_ROWS},
};

pub type SharedMainBoard = Rc<RefCell<MainBoard>>;

pub struct MainBoard {
	cells: VecDeque<Vec<Option<TetrominoKind>>>,
	pub line_clear: LineClear,
}

impl MainBoard {
	fn new() -> Self {
		Self {
			cells: VecDeque::from_iter(vec![
				vec![None; MAIN_BOARD_COLS];
				MAIN_BOARD_ROWS
			]),
			line_clear: LineClear::default(),
		}
	}

	pub fn new_shared() -> SharedMainBoard {
		Rc::new(RefCell::new(Self::new()))
	}

	pub fn lock_tetromino(&mut self, tetromino: &Tetromino) {
		for p in tetromino.position.to_usize_points() {
			self.cells[p.y][p.x] = Some(tetromino.kind);
		}
	}

	pub fn is_collision(&self, position: &Position) -> bool {
		position
			.to_usize_points()
			.iter()
			.any(|p| self.cells[p.y][p.x].is_some())
	}

	pub fn check_line_clear(&mut self) -> usize {
		for (i, line) in self.cells.iter().enumerate() {
			if line.iter().any(|kind| kind.is_none()) {
				continue;
			}
			self.line_clear.lines.push(i);
		}
		let num = self.line_clear.lines.len();
		if num != 0 {
			self.line_clear.status = LineClearStatus::Pending;
		}
		num
	}

	fn clear_cell(&mut self) {
		for (y, line) in self.cells.iter_mut().enumerate() {
			if !self.line_clear.lines.contains(&y) {
				continue;
			}
			for (x, cell) in line.iter_mut().enumerate() {
				if x == self.line_clear.curosr {
					*cell = None;
					// todo: confetti
				}
			}
		}
	}

	fn gen_new_lines(&mut self) {
		for line in &self.line_clear.lines {
			self.cells.remove(*line);
			self.cells.push_front(vec![None; MAIN_BOARD_COLS]);
		}
		self.line_clear.lines.clear();
	}

	pub fn update_line_clear(&mut self) {
		// todo: confetti
		if self.line_clear.status != LineClearStatus::Pending {
			return;
		}
		self.clear_cell();
		if self.line_clear.curosr >= MAIN_BOARD_COLS {
			self.line_clear.curosr = 0;
			self.line_clear.status = LineClearStatus::Done;
			self.gen_new_lines();
		} else {
			self.line_clear.curosr += 1;
		}
	}
}

impl Board for MainBoard {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind> {
		self.cells[y][x].as_ref()
	}
}

#[derive(Default)]
pub struct LineClear {
	pub status: LineClearStatus,
	lines: Vec<usize>,
	curosr: usize,
}

#[derive(PartialEq, Eq, Default)]
pub enum LineClearStatus {
	#[default]
	None,
	Pending,
	Done,
}
