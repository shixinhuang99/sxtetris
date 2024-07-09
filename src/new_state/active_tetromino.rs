use std::rc::Rc;

use super::main_board::MainBoard;
use crate::{
	consts::{MAIN_BOARD_BUFFER_ROWS, MAIN_BOARD_COLS, MAIN_BOARD_ROWS},
	core::{position::Position, tetromino_kind::TetrominoKind},
};

const MAX_Y: i8 = MAIN_BOARD_ROWS as i8 - 1;
const MAX_X: i8 = MAIN_BOARD_COLS as i8 - 1;
const MIN_Y: i8 = MAIN_BOARD_BUFFER_ROWS as i8;

pub struct ActiveTetromino {
	kind: TetrominoKind,
	position: Position,
	orientation: Orientation,
	board: Rc<MainBoard>,
}

impl ActiveTetromino {
	pub fn new(kind: TetrominoKind, board: Rc<MainBoard>) -> Self {
		let orientation = Orientation::N;
		let mut position = kind.init_position(orientation.into());

		position.update(|p| {
			p.x += 3;
			p.y += MIN_Y;
		});

		Self {
			kind,
			position,
			orientation,
			board,
		}
	}

	pub fn is_touch_bottom(&self) -> bool {
		self.position.iter().any(|p| p.y >= MAX_Y)
	}

	pub fn is_touch_left(&self) -> bool {
		self.position.iter().any(|p| p.x <= 0)
	}

	pub fn is_touch_right(&self) -> bool {
		self.position.iter().any(|p| p.x >= MAX_X)
	}

	pub fn is_outside_the_board(&self) -> bool {
		self.position
			.iter()
			.any(|p| p.x < 0 || p.x > MAX_X || p.y < 0 || p.y > MAX_Y)
	}

	pub fn is_outside_the_visible(&self) -> bool {
		self.position
			.iter()
			.any(|p| p.x < 0 || p.x > MAX_X || p.y < MIN_Y || p.y > MAX_Y)
	}
}

#[derive(Clone, Copy)]
enum Orientation {
	N,
	E,
	W,
	S,
}

impl From<usize> for Orientation {
	fn from(value: usize) -> Self {
		match value {
			0 => Orientation::N,
			1 => Orientation::E,
			2 => Orientation::S,
			3 => Orientation::W,
			_ => unreachable!(),
		}
	}
}

impl From<Orientation> for usize {
	fn from(value: Orientation) -> Self {
		match value {
			Orientation::N => 0,
			Orientation::E => 1,
			Orientation::S => 2,
			Orientation::W => 3,
		}
	}
}
