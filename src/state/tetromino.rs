use super::position::Position;
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
			TetrominoKind::Ghost => 'G',
			TetrominoKind::None => '-',
		}
	}
}

#[derive(PartialEq)]
pub enum TetrominoAction {
	Left,
	Right,
	SoftDrop,
	HardDrop,
}

#[derive(Clone)]
enum RotateDeg {
	Zero,
	R,
	L,
	Two,
}

#[derive(Clone)]
pub struct Tetromino {
	pub kind: TetrominoKind,
	pub position: Position,
	rotate_deg: RotateDeg,
}

impl Tetromino {
	pub fn new(kind: TetrominoKind) -> Self {
		let mut tm = Self::new_preview(kind);

		tm.position.update(|p| p.1 += BOARD_VISIBLE_Y_LEN);

		tm
	}

	pub fn new_preview(kind: TetrominoKind) -> Self {
		let points = match kind {
			TetrominoKind::I => [(3, 1), (4, 1), (5, 1), (6, 1)],
			TetrominoKind::J => [(3, 0), (3, 1), (4, 1), (5, 1)],
			TetrominoKind::L => [(5, 0), (3, 1), (4, 1), (5, 1)],
			TetrominoKind::O => [(4, 0), (5, 0), (4, 1), (5, 1)],
			TetrominoKind::S => [(4, 0), (5, 0), (3, 1), (4, 1)],
			TetrominoKind::T => [(4, 0), (3, 1), (4, 1), (5, 1)],
			TetrominoKind::Z => [(3, 0), (4, 0), (4, 1), (5, 1)],
			TetrominoKind::None | TetrominoKind::Ghost => [(0, 0); 4],
		};

		Self {
			kind,
			position: Position::new(points),
			rotate_deg: RotateDeg::Zero,
		}
	}

	pub fn up(&mut self) -> bool {
		if self.position.is_touched_top() {
			true
		} else {
			self.position.update(|p| p.1 -= 1);
			false
		}
	}

	pub fn down(&mut self) -> bool {
		if self.position.is_touched_bottom() {
			true
		} else {
			self.position.update(|p| p.1 += 1);
			false
		}
	}

	pub fn left(&mut self) -> bool {
		if self.position.is_touched_left() {
			true
		} else {
			self.position.update(|p| p.0 -= 1);
			false
		}
	}

	pub fn right(&mut self) -> bool {
		if self.position.is_touched_right() {
			true
		} else {
			self.position.update(|p| p.0 += 1);
			false
		}
	}

	pub fn same_position(&self, other: &Self) -> bool {
		self.position == other.position
	}

	fn rotate(&mut self) {
		unimplemented!();
	}
}

impl Default for Tetromino {
	fn default() -> Self {
		Tetromino::new(TetrominoKind::None)
	}
}
