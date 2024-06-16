use super::point::{Point, Points};
use crate::consts::BOARD_VISIBLE_Y_LEN;

#[derive(Clone, Copy, PartialEq)]
pub enum TetrominoKind {
	I,
	/// ```
	/// []   
	/// [][][]
	/// ```
	J,
	/// ```
	///     []
	/// [][][]
	/// ```
	L,
	O,
	/// ```
	///   [][]
	/// [][]
	/// ```
	S,
	T,
	/// ```
	/// [][]
	///   [][]
	/// ```
	Z,
	None,
	Ghost,
}

impl TetrominoKind {
	fn init_points(&self, idx: usize) -> [Point; 4] {
		match self {
			TetrominoKind::I => rotate_map::I[idx],
			TetrominoKind::J => rotate_map::J[idx],
			TetrominoKind::L => rotate_map::L[idx],
			TetrominoKind::O => rotate_map::O[idx],
			TetrominoKind::S => rotate_map::S[idx],
			TetrominoKind::T => rotate_map::T[idx],
			TetrominoKind::Z => rotate_map::Z[idx],
			_ => [(0, 0); 4],
		}
	}
}

impl From<char> for TetrominoKind {
	fn from(value: char) -> Self {
		match value {
			'I' => TetrominoKind::I,
			'J' => TetrominoKind::J,
			'L' => TetrominoKind::L,
			'O' => TetrominoKind::O,
			'S' => TetrominoKind::S,
			'T' => TetrominoKind::T,
			'Z' => TetrominoKind::Z,
			_ => TetrominoKind::None,
		}
	}
}

impl From<TetrominoKind> for char {
	fn from(value: TetrominoKind) -> Self {
		match value {
			TetrominoKind::I => 'I',
			TetrominoKind::J => 'J',
			TetrominoKind::L => 'L',
			TetrominoKind::O => 'O',
			TetrominoKind::S => 'S',
			TetrominoKind::T => 'T',
			TetrominoKind::Z => 'Z',
			TetrominoKind::None | TetrominoKind::Ghost => '-',
		}
	}
}

#[derive(PartialEq)]
pub enum TetrominoAction {
	Left,
	Right,
	SoftDrop,
	HardDrop,
	RotateRight,
	RotateLeft,
}

#[derive(Clone, Copy)]
enum RotateDeg {
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

#[derive(Clone)]
pub struct Tetromino {
	pub kind: TetrominoKind,
	pub points: Points,
	rotate_deg: RotateDeg,
}

impl Tetromino {
	pub fn new(kind: TetrominoKind) -> Self {
		let mut tm = Self::new_preview(kind);

		tm.points.update(|p| p.1 += BOARD_VISIBLE_Y_LEN as i32);

		tm
	}

	pub fn new_preview(kind: TetrominoKind) -> Self {
		let mut points = Points::new(kind.init_points(0));

		points.update(|p| p.0 += 3);

		Self {
			kind,
			points,
			rotate_deg: RotateDeg::Zero,
		}
	}

	pub fn walk(&mut self, action: &TetrominoAction) -> bool {
		if match action {
			TetrominoAction::SoftDrop => self.points.is_touched_bottom(),
			TetrominoAction::Left => self.points.is_touched_left(),
			TetrominoAction::Right => self.points.is_touched_right(),
			_ => unreachable!(),
		} {
			false
		} else {
			match action {
				TetrominoAction::SoftDrop => self.points.update(|p| p.1 += 1),
				TetrominoAction::Left => self.points.update(|p| p.0 -= 1),
				TetrominoAction::Right => self.points.update(|p| p.0 += 1),
				_ => unreachable!(),
			}
			true
		}
	}

	pub fn same_position(&self, other: &Self) -> bool {
		self.points == other.points
	}

	pub fn rotate<F>(
		&mut self,
		action: &TetrominoAction,
		is_collision: F,
	) -> bool
	where
		F: Fn(&Points) -> bool,
	{
		use RotateDeg::*;

		let init_points = self.kind.init_points(self.rotate_deg.into());

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

		let mut next_points =
			Points::new(self.kind.init_points(next_deg.into()));

		for (i, v) in diff.into_iter().enumerate() {
			next_points.value[i].0 += v.0;
			next_points.value[i].1 += v.1;
		}

		#[cfg(feature = "_dev")]
		log::trace!("next points: {:?}", next_points.value);

		let mut ok = false;

		if next_points.is_out_of_board() || is_collision(&next_points) {
			let kick_offest = match (&self.rotate_deg, next_deg) {
				(Zero, R) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::ZERO_R
					} else {
						kick_map::jlstz::ZERO_R
					}
				}
				(R, Zero) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::R_ZERO
					} else {
						kick_map::jlstz::R_ZERO
					}
				}
				(R, Two) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::R_TWO
					} else {
						kick_map::jlstz::R_TWO
					}
				}
				(Two, R) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::TWO_R
					} else {
						kick_map::jlstz::TWO_R
					}
				}
				(Two, L) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::TWO_L
					} else {
						kick_map::jlstz::TWO_L
					}
				}
				(L, Two) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::L_TWO
					} else {
						kick_map::jlstz::L_TWO
					}
				}
				(L, Zero) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::L_ZERO
					} else {
						kick_map::jlstz::L_ZERO
					}
				}
				(Zero, L) => {
					if self.kind == TetrominoKind::I {
						kick_map::i::ZERO_L
					} else {
						kick_map::jlstz::ZERO_L
					}
				}
				_ => unreachable!(),
			};

			for offest in kick_offest {
				let mut points = next_points.clone();

				points.update(|p| {
					p.0 += offest.0;
					p.1 += offest.1;
				});

				if points.is_out_of_board() || is_collision(&points) {
					continue;
				}

				ok = self.points != points;
				self.points = points;
				self.rotate_deg = next_deg;
				break;
			}
		} else {
			ok = self.points != next_points;
			self.points = next_points;
			self.rotate_deg = next_deg;
		}

		ok
	}

	pub fn stringify(&self) -> String {
		let mut content = String::from("#tetromino\n");

		content.push(self.kind.into());

		for p in &self.points.value {
			content.push_str(&format!(" {} {}", p.0, p.1));
		}

		content.push_str(&format!(" {}\n", usize::from(self.rotate_deg)));

		content
	}

	pub fn read_save(&mut self, source: String) {
		let chunks: Vec<&str> = source.split_ascii_whitespace().collect();

		if chunks.len() != 10 {
			return;
		}

		self.kind = TetrominoKind::from(chunks[0].chars().next().unwrap());

		for (i, point) in chunks[1..9].chunks(2).enumerate() {
			self.points.value[i].0 = point[0].parse::<i32>().unwrap();
			self.points.value[i].1 = point[1].parse::<i32>().unwrap();
		}

		self.rotate_deg = RotateDeg::from(chunks[9].parse::<usize>().unwrap());
	}
}

mod rotate_map {
	use super::Point;

	type Map = [[Point; 4]; 4];

	pub const I: Map = [
		[(0, 1), (1, 1), (2, 1), (3, 1)],
		[(2, 0), (2, 1), (2, 2), (2, 3)],
		[(0, 2), (1, 2), (2, 2), (3, 2)],
		[(1, 0), (1, 1), (1, 2), (1, 3)],
	];

	pub const J: Map = [
		[(0, 0), (0, 1), (1, 1), (2, 1)],
		[(2, 0), (1, 0), (1, 1), (1, 2)],
		[(2, 2), (2, 1), (1, 1), (0, 1)],
		[(0, 2), (1, 2), (1, 1), (1, 0)],
	];

	pub const L: Map = [
		[(2, 0), (0, 1), (1, 1), (2, 1)],
		[(2, 2), (1, 0), (1, 1), (1, 2)],
		[(0, 2), (2, 1), (1, 1), (0, 1)],
		[(0, 0), (1, 2), (1, 1), (1, 0)],
	];

	pub const S: Map = [
		[(1, 0), (2, 0), (0, 1), (1, 1)],
		[(2, 1), (2, 2), (1, 0), (1, 1)],
		[(1, 2), (0, 2), (2, 1), (1, 1)],
		[(0, 1), (0, 0), (1, 2), (1, 1)],
	];

	pub const T: Map = [
		[(1, 0), (0, 1), (1, 1), (2, 1)],
		[(2, 1), (1, 0), (1, 1), (1, 2)],
		[(1, 2), (2, 1), (1, 1), (0, 1)],
		[(0, 1), (1, 2), (1, 1), (1, 0)],
	];

	pub const Z: Map = [
		[(0, 0), (1, 0), (1, 1), (2, 1)],
		[(2, 0), (2, 1), (1, 1), (1, 2)],
		[(2, 2), (1, 2), (1, 1), (0, 1)],
		[(0, 2), (0, 1), (1, 1), (1, 0)],
	];

	pub const O: Map = [
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
	];
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
