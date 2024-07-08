use serde::{Deserialize, Serialize};

use super::{
	point::{Point, Points},
	tetromino_type::TetrominoType,
};
use crate::consts::BOARD_VISIBLE_ROWS;

const BOARD_VISIBLE_ROWS_I32: i32 = BOARD_VISIBLE_ROWS as i32;

#[derive(PartialEq)]
pub enum TetrominoAction {
	Left,
	Right,
	SoftDrop,
	HardDrop,
	RotateRight,
	RotateLeft,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RotateDeg {
	Zero,
	R,
	L,
	Two,
}

impl From<usize> for RotateDeg {
	fn from(value: usize) -> Self {
		match value {
			0 => RotateDeg::Zero,
			1 => RotateDeg::R,
			3 => RotateDeg::L,
			2 => RotateDeg::Two,
			_ => unreachable!(),
		}
	}
}

impl From<RotateDeg> for usize {
	fn from(value: RotateDeg) -> Self {
		match value {
			RotateDeg::Zero => 0,
			RotateDeg::R => 1,
			RotateDeg::L => 3,
			RotateDeg::Two => 2,
		}
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Tetromino {
	pub tm_type: TetrominoType,
	pub points: Points,
	pub rotate_deg: RotateDeg,
}

impl Tetromino {
	pub fn new(tm_type: TetrominoType) -> Self {
		let mut tm = Self::new_preview(tm_type);

		tm.points.update(|p| p.1 += BOARD_VISIBLE_ROWS_I32);

		tm
	}

	pub fn new_preview(tm_type: TetrominoType) -> Self {
		let mut points = Points::new(tm_type.init_points(0));

		points.update(|p| p.0 += 3);

		Self {
			tm_type,
			points,
			rotate_deg: RotateDeg::Zero,
		}
	}

	pub fn can_move<F>(
		&self,
		action: &TetrominoAction,
		is_collision: F,
	) -> Option<Points>
	where
		F: Fn(&Points) -> bool,
	{
		let mut points = self.points.clone();

		let moved = match action {
			TetrominoAction::SoftDrop => {
				if points.is_touched_bottom() {
					false
				} else {
					points.update(|p| p.1 += 1);
					true
				}
			}
			TetrominoAction::Left => {
				if points.is_touched_left() {
					false
				} else {
					points.update(|p| p.0 -= 1);
					true
				}
			}
			TetrominoAction::Right => {
				if points.is_touched_right() {
					false
				} else {
					points.update(|p| p.0 += 1);
					true
				}
			}
			_ => unreachable!(),
		};

		if !moved || is_collision(&points) {
			None
		} else {
			Some(points)
		}
	}

	pub fn can_rotate<F>(
		&self,
		action: &TetrominoAction,
		active_tm_points: &Points,
		is_collision: F,
	) -> Option<(Points, RotateDeg)>
	where
		F: Fn(&Points, &Points) -> bool,
	{
		use RotateDeg::*;

		let init_points = self.tm_type.init_points(self.rotate_deg.into());

		let mut diff: [Point; 4] = [(0, 0); 4];

		for i in 0..4 {
			diff[i].0 = self.points.value[i].0 - init_points[i].0;
			diff[i].1 = self.points.value[i].1 - init_points[i].1;
		}

		let next_deg = match action {
			TetrominoAction::RotateRight => {
				match self.rotate_deg {
					Zero => R,
					R => Two,
					Two => L,
					L => Zero,
				}
			}
			TetrominoAction::RotateLeft => {
				match self.rotate_deg {
					Zero => L,
					L => Two,
					Two => R,
					R => Zero,
				}
			}
			_ => unreachable!(),
		};

		let mut fisrt_rotate_points =
			Points::new(self.tm_type.init_points(next_deg.into()));

		for (i, v) in diff.into_iter().enumerate() {
			fisrt_rotate_points.value[i].0 += v.0;
			fisrt_rotate_points.value[i].1 += v.1;
		}

		if fisrt_rotate_points.is_out_of_board()
			|| is_collision(&fisrt_rotate_points, active_tm_points)
		{
			let kick_offest = match (&self.rotate_deg, next_deg) {
				(Zero, R) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::ZERO_R
					} else {
						kick_map::jlstz::ZERO_R
					}
				}
				(R, Zero) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::R_ZERO
					} else {
						kick_map::jlstz::R_ZERO
					}
				}
				(R, Two) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::R_TWO
					} else {
						kick_map::jlstz::R_TWO
					}
				}
				(Two, R) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::TWO_R
					} else {
						kick_map::jlstz::TWO_R
					}
				}
				(Two, L) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::TWO_L
					} else {
						kick_map::jlstz::TWO_L
					}
				}
				(L, Two) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::L_TWO
					} else {
						kick_map::jlstz::L_TWO
					}
				}
				(L, Zero) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::L_ZERO
					} else {
						kick_map::jlstz::L_ZERO
					}
				}
				(Zero, L) => {
					if self.tm_type == TetrominoType::I {
						kick_map::i::ZERO_L
					} else {
						kick_map::jlstz::ZERO_L
					}
				}
				_ => unreachable!(),
			};

			for offest in kick_offest {
				let mut kick_points = fisrt_rotate_points.clone();

				kick_points.update(|p| {
					p.0 += offest.0;
					p.1 += offest.1;
				});

				if kick_points.is_out_of_board()
					|| is_collision(&kick_points, active_tm_points)
					|| is_collision(&kick_points, &fisrt_rotate_points)
				{
					continue;
				}

				if self.points != kick_points {
					return Some((kick_points, next_deg));
				}

				break;
			}

			return None;
		}

		if self.points != fisrt_rotate_points {
			Some((fisrt_rotate_points, next_deg))
		} else {
			None
		}
	}

	pub fn same_position(&self, other: &Self) -> bool {
		self.points == other.points
	}
}

mod kick_map {
	use super::Point;

	type KickOffest = [Point; 4];

	pub mod jlstz {
		use super::KickOffest;

		pub const ZERO_R: KickOffest = [(-1, 0), (-1, 1), (0, -2), (-1, -2)];

		pub const R_ZERO: KickOffest = [(1, 0), (1, -1), (0, 2), (1, 2)];

		pub const R_TWO: KickOffest = [(1, 0), (1, -1), (0, 2), (1, 2)];

		pub const TWO_R: KickOffest = [(-1, 0), (-1, 1), (0, -2), (-1, -2)];

		pub const TWO_L: KickOffest = [(1, 0), (1, 1), (0, -2), (1, -2)];

		pub const L_TWO: KickOffest = [(-1, 0), (-1, -1), (0, 2), (-1, 2)];

		pub const L_ZERO: KickOffest = [(-1, 0), (-1, -1), (0, 2), (-1, 2)];

		pub const ZERO_L: KickOffest = [(1, 0), (1, 1), (0, -2), (1, -2)];
	}

	pub mod i {
		use super::KickOffest;

		pub const ZERO_R: KickOffest = [(-2, 0), (1, 0), (-2, -1), (1, 2)];

		pub const R_ZERO: KickOffest = [(2, 0), (-1, 0), (2, 1), (-1, -2)];

		pub const R_TWO: KickOffest = [(-1, 0), (2, 0), (-1, 2), (2, -1)];

		pub const TWO_R: KickOffest = [(1, 0), (-2, 0), (1, -2), (-2, 1)];

		pub const TWO_L: KickOffest = [(2, 0), (-1, 0), (2, 1), (-1, -2)];

		pub const L_TWO: KickOffest = [(-2, 0), (1, 0), (-2, -1), (1, 2)];

		pub const L_ZERO: KickOffest = [(1, 0), (-2, 0), (1, -2), (-2, 1)];

		pub const ZERO_L: KickOffest = [(-1, 0), (2, 0), (-1, 2), (2, -1)];
	}
}
