use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{particles::Particles, Tetromino};
use crate::{
	common::{Board, Position, Reset, TetrominoKind},
	consts::{MAIN_BOARD_COLS, MAIN_BOARD_ROWS},
	global::{global_audio, Sound},
};

pub type SharedMainBoard = Rc<RefCell<MainBoard>>;

#[derive(Clone, Deserialize, Serialize)]
pub struct MainBoard {
	cells: VecDeque<Vec<Option<TetrominoKind>>>,
	#[serde(skip)]
	pub line_clear: LineClear,
	#[serde(skip)]
	pub particles: Particles,
}

#[derive(Clone, Default)]
pub struct LineClear {
	pub in_progress: bool,
	lines: Vec<usize>,
	curosr: usize,
}

impl MainBoard {
	fn new() -> Self {
		Self {
			cells: VecDeque::from_iter(vec![
				vec![None; MAIN_BOARD_COLS];
				MAIN_BOARD_ROWS
			]),
			line_clear: LineClear::default(),
			particles: Particles::default(),
		}
	}

	pub fn new_shared() -> SharedMainBoard {
		Rc::new(RefCell::new(Self::new()))
	}

	pub fn lock_tetromino(&mut self, tetromino: &Tetromino) -> usize {
		for p in tetromino.position.to_usize_points() {
			self.cells[p.y][p.x] = Some(tetromino.kind);
		}

		for (i, line) in self.cells.iter().enumerate() {
			if line.iter().any(|kind| kind.is_none()) {
				continue;
			}
			self.line_clear.lines.push(i);
		}

		global_audio(|audio| audio.play_sound(Sound::Lock));

		let num = self.line_clear.lines.len();
		if num != 0 {
			self.line_clear.in_progress = true;
			global_audio(|audio| audio.play_sound(Sound::Clear));
		}

		num
	}

	pub fn is_collision(&self, position: &Position) -> bool {
		position
			.to_usize_points()
			.iter()
			.any(|p| self.cells[p.y][p.x].is_some())
	}

	fn clear_cell(&mut self) {
		for line in &self.line_clear.lines {
			self.cells[*line][self.line_clear.curosr] = None;
			self.particles.push_point(self.line_clear.curosr, *line);
		}
	}

	pub fn update_line_clear(&mut self) -> bool {
		self.clear_cell();
		self.line_clear.curosr += 1;

		if self.line_clear.curosr >= MAIN_BOARD_COLS {
			self.line_clear.curosr = 0;
			self.line_clear.in_progress = false;
			for line in &self.line_clear.lines {
				self.cells.remove(*line);
				self.cells.push_front(vec![None; MAIN_BOARD_COLS]);
			}
			self.line_clear.lines.clear();

			true
		} else {
			false
		}
	}
}

impl Board for MainBoard {
	fn get_kind(&self, x: usize, y: usize) -> Option<&TetrominoKind> {
		self.cells[y][x].as_ref()
	}
}

impl Reset for MainBoard {
	fn reset(&mut self) {
		*self = Self::new();
	}
}
