use std::{cell::RefCell, rc::Rc};

use super::MainBoard;
use crate::{
	consts::MAIN_BOARD_BUFFER_ROWS,
	core::{Position, TetrominoKind},
};

pub struct Tetromino {
	pub kind: TetrominoKind,
	pub position: Position,
	pub blink: bool,
	orientation: Orientation,
	board: Rc<RefCell<MainBoard>>,
}

impl Tetromino {
	pub fn new(board: Rc<RefCell<MainBoard>>) -> Self {
		Self {
			kind: TetrominoKind::I,
			position: Position::default(),
			blink: false,
			orientation: Orientation::N,
			board,
		}
	}

	pub fn into_new(self, kind: TetrominoKind) -> Self {
		let mut tetromino = Tetromino::new(self.board);

		tetromino.kind = kind;
		tetromino.position = kind.init_position(Orientation::N.into());
		tetromino.position.update(|p| {
			p.x += 3;
			p.y += MAIN_BOARD_BUFFER_ROWS as i8;
		});

		tetromino
	}

	fn walk(&mut self, action: &TetrominoAction) {
		let mut position = self.position.clone();

		let moved = match action {
			TetrominoAction::SoftDrop => {
				if position.is_touch_bottom() {
					false
				} else {
					position.update(|p| p.y += 1);
					true
				}
			}
			TetrominoAction::Left => {
				if position.is_touch_left() {
					false
				} else {
					position.update(|p| p.x -= 1);
					true
				}
			}
			TetrominoAction::Right => {
				if position.is_touch_right() {
					false
				} else {
					position.update(|p| p.x += 1);
					true
				}
			}
			_ => unreachable!(),
		};

		if !moved || self.board.borrow().is_collision(&position) {
			return;
		}

		self.position = position;
	}

	fn rotate(&mut self, action: &TetrominoAction) {
		use Orientation::*;

		let init_position = self.kind.init_position(self.orientation.into());
		let diff = self.position.clone() - init_position;

		let next_orientation = match action {
			TetrominoAction::RotateRight => {
				match self.orientation {
					N => E,
					E => S,
					S => W,
					W => N,
				}
			}
			TetrominoAction::RotateLeft => {
				match self.orientation {
					N => W,
					W => S,
					S => E,
					E => N,
				}
			}
			_ => unreachable!(),
		};

		let init_position = self.kind.init_position(next_orientation.into());
		let rotate_position = init_position + diff;

		if rotate_position.is_outside_the_board()
			|| self.board.borrow().is_collision(&rotate_position)
		{
			let kick_offest = match (&self.orientation, next_orientation) {
				(N, E) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::NE
					} else {
						kick_map_jlstz::NE
					}
				}
				(E, N) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::EN
					} else {
						kick_map_jlstz::EN
					}
				}
				(E, S) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::ES
					} else {
						kick_map_jlstz::ES
					}
				}
				(S, E) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::SE
					} else {
						kick_map_jlstz::SE
					}
				}
				(S, W) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::SW
					} else {
						kick_map_jlstz::SW
					}
				}
				(W, S) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::WS
					} else {
						kick_map_jlstz::WS
					}
				}
				(W, N) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::WN
					} else {
						kick_map_jlstz::WN
					}
				}
				(N, W) => {
					if self.kind == TetrominoKind::I {
						kick_map_i::NW
					} else {
						kick_map_jlstz::NW
					}
				}
				_ => unreachable!(),
			};

			for offest in kick_offest.into_iter() {
				let kick_position = rotate_position.clone() + offest;

				if kick_position.is_outside_the_board()
					|| self.board.borrow().is_collision(&kick_position)
				{
					continue;
				}

				self.position = kick_position;
				self.orientation = next_orientation;

				return;
			}

			return;
		}

		self.position = rotate_position;
		self.orientation = next_orientation;
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

#[derive(PartialEq, Eq)]
pub enum TetrominoAction {
	Left,
	Right,
	SoftDrop,
	HardDrop,
	RotateRight,
	RotateLeft,
}

mod kick_map_jlstz {
	use super::Position;

	pub const NE: Position =
		Position::new([(-1, 0), (-1, 1), (0, -2), (-1, -2)]);

	pub const EN: Position = Position::new([(1, 0), (1, -1), (0, 2), (1, 2)]);

	pub const ES: Position = Position::new([(1, 0), (1, -1), (0, 2), (1, 2)]);

	pub const SE: Position =
		Position::new([(-1, 0), (-1, 1), (0, -2), (-1, -2)]);

	pub const SW: Position = Position::new([(1, 0), (1, 1), (0, -2), (1, -2)]);

	pub const WS: Position =
		Position::new([(-1, 0), (-1, -1), (0, 2), (-1, 2)]);

	pub const WN: Position =
		Position::new([(-1, 0), (-1, -1), (0, 2), (-1, 2)]);

	pub const NW: Position = Position::new([(1, 0), (1, 1), (0, -2), (1, -2)]);
}

mod kick_map_i {
	use super::Position;

	pub const NE: Position = Position::new([(-2, 0), (1, 0), (-2, -1), (1, 2)]);

	pub const EN: Position = Position::new([(2, 0), (-1, 0), (2, 1), (-1, -2)]);

	pub const ES: Position = Position::new([(-1, 0), (2, 0), (-1, 2), (2, -1)]);

	pub const SE: Position = Position::new([(1, 0), (-2, 0), (1, -2), (-2, 1)]);

	pub const SW: Position = Position::new([(2, 0), (-1, 0), (2, 1), (-1, -2)]);

	pub const WS: Position = Position::new([(-2, 0), (1, 0), (-2, -1), (1, 2)]);

	pub const WN: Position = Position::new([(1, 0), (-2, 0), (1, -2), (-2, 1)]);

	pub const NW: Position = Position::new([(-1, 0), (2, 0), (-1, 2), (2, -1)]);
}
