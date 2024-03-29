use crate::consts::{MATRIX_X_LEN, MATRIX_Y_LEN, MATRIX_Y_VISIBLE_LEN};

const MAX_X: usize = MATRIX_X_LEN - 1;
const MAX_Y: usize = MATRIX_Y_LEN - 1;

#[derive(Clone, Copy)]
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
	pub pos: [(usize, usize); 4],
	rotate_deg: RotateDeg,
}

impl Tetromino {
	pub fn new(kind: TetrominoKind) -> Self {
		let mut this = Self::new_without_offest(kind);

		for p in &mut this.pos {
			p.1 += MATRIX_Y_VISIBLE_LEN;
		}

		this
	}

	pub fn new_without_offest(kind: TetrominoKind) -> Self {
		let pos = match kind {
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
			pos,
			rotate_deg: RotateDeg::Zero,
		}
	}

	pub fn up(&mut self) -> bool {
		if self.pos.iter().any(|p| p.1 == 0) {
			return true;
		}
		for pos in &mut self.pos {
			pos.1 -= 1;
		}
		false
	}

	pub fn down(&mut self) -> bool {
		if self.pos.iter().any(|p| p.1 == MAX_Y) {
			return true;
		}
		for pos in &mut self.pos {
			pos.1 += 1;
		}
		false
	}

	pub fn left(&mut self) -> bool {
		if self.pos.iter().any(|p| p.0 == 0) {
			return true;
		}
		for pos in &mut self.pos {
			pos.0 -= 1;
		}
		false
	}

	pub fn right(&mut self) -> bool {
		if self.pos.iter().any(|p| p.0 == MAX_X) {
			return true;
		}
		for pos in &mut self.pos {
			pos.0 += 1;
		}
		false
	}

	pub fn same_position(&self, rhs: &Self) -> bool {
		self.pos
			.iter()
			.enumerate()
			.all(|(idx, p)| p == &rhs.pos[idx])
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
