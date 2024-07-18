use serde::{Deserialize, Serialize};

use super::{MainBoard, SharedMainBoard};
use crate::{
	common::{pos, Position, TetrominoKind},
	consts::MAIN_BOARD_BUFFER_ROWS,
};

#[derive(Clone, Deserialize, Serialize)]
pub struct Tetromino {
	pub kind: TetrominoKind,
	pub position: Position,
	orientation: Orientation,
	#[serde(skip)]
	pub blink: bool,
	#[serde(skip, default = "MainBoard::new_shared")]
	board: SharedMainBoard,
}

impl Tetromino {
	pub fn new(board: SharedMainBoard) -> Self {
		Self {
			kind: TetrominoKind::default(),
			position: Position::default(),
			blink: false,
			orientation: Orientation::default(),
			board,
		}
	}

	pub fn set_next(&mut self, kind: TetrominoKind) {
		self.kind = kind;
		self.orientation = Orientation::default();
		self.position = kind.init_position(self.orientation.into());
		self.position.update(|p| {
			p.x += 3;
			p.y += MAIN_BOARD_BUFFER_ROWS as i8;
		});
		self.blink = false;
	}

	pub fn set_board(&mut self, board: SharedMainBoard) {
		self.board = board;
	}

	pub fn walk(&mut self, action: TetrominoAction) -> bool {
		let mut position = self.position.clone();

		let can_walk = match action {
			TetrominoAction::SoftDrop => {
				if position.is_touch_bottom() {
					false
				} else {
					position.update(|p| p.y += 1);
					true
				}
			}
			TetrominoAction::WalkLeft => {
				if position.is_touch_left() {
					false
				} else {
					position.update(|p| p.x -= 1);
					true
				}
			}
			TetrominoAction::WalkRight => {
				if position.is_touch_right() {
					false
				} else {
					position.update(|p| p.x += 1);
					true
				}
			}
			_ => unreachable!(),
		};

		if !can_walk || self.board.borrow().is_collision(&position) {
			false
		} else {
			self.position = position;
			true
		}
	}

	pub fn rotate(&mut self, action: TetrominoAction) -> bool {
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

		let mut rotated = false;

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

				if self.position != kick_position {
					self.position = kick_position;
					self.orientation = next_orientation;
					rotated = true;
				}

				break;
			}
		} else if self.position != rotate_position {
			self.position = rotate_position;
			self.orientation = next_orientation;
			rotated = true;
		}

		rotated
	}
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
enum Orientation {
	#[default]
	N,
	E,
	W,
	S,
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

pub enum TetrominoAction {
	WalkLeft,
	WalkRight,
	SoftDrop,
	RotateRight,
	RotateLeft,
}

mod kick_map_jlstz {
	use super::{pos, Position};

	pub const NE: Position = pos([(-1, 0), (-1, 1), (0, -2), (-1, -2)]);

	pub const EN: Position = pos([(1, 0), (1, -1), (0, 2), (1, 2)]);

	pub const ES: Position = pos([(1, 0), (1, -1), (0, 2), (1, 2)]);

	pub const SE: Position = pos([(-1, 0), (-1, 1), (0, -2), (-1, -2)]);

	pub const SW: Position = pos([(1, 0), (1, 1), (0, -2), (1, -2)]);

	pub const WS: Position = pos([(-1, 0), (-1, -1), (0, 2), (-1, 2)]);

	pub const WN: Position = pos([(-1, 0), (-1, -1), (0, 2), (-1, 2)]);

	pub const NW: Position = pos([(1, 0), (1, 1), (0, -2), (1, -2)]);
}

mod kick_map_i {
	use super::{pos, Position};

	pub const NE: Position = pos([(-2, 0), (1, 0), (-2, -1), (1, 2)]);

	pub const EN: Position = pos([(2, 0), (-1, 0), (2, 1), (-1, -2)]);

	pub const ES: Position = pos([(-1, 0), (2, 0), (-1, 2), (2, -1)]);

	pub const SE: Position = pos([(1, 0), (-2, 0), (1, -2), (-2, 1)]);

	pub const SW: Position = pos([(2, 0), (-1, 0), (2, 1), (-1, -2)]);

	pub const WS: Position = pos([(-2, 0), (1, 0), (-2, -1), (1, 2)]);

	pub const WN: Position = pos([(1, 0), (-2, 0), (1, -2), (-2, 1)]);

	pub const NW: Position = pos([(-1, 0), (2, 0), (-1, 2), (2, -1)]);
}
