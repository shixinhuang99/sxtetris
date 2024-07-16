use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use super::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
}

impl TetrominoKind {
	pub fn init_position(&self, idx: usize) -> Position {
		let position = match self {
			TetrominoKind::I => &position_map::I[idx],
			TetrominoKind::J => &position_map::J[idx],
			TetrominoKind::L => &position_map::L[idx],
			TetrominoKind::O => &position_map::O[idx],
			TetrominoKind::S => &position_map::S[idx],
			TetrominoKind::T => &position_map::T[idx],
			TetrominoKind::Z => &position_map::Z[idx],
		};

		position.clone()
	}

	pub fn color(&self) -> Color {
		match self {
			TetrominoKind::I => color::cyan(),
			TetrominoKind::J => color::blue(),
			TetrominoKind::L => color::orange(),
			TetrominoKind::O => color::yellow(),
			TetrominoKind::S => color::green(),
			TetrominoKind::T => color::purple(),
			TetrominoKind::Z => color::red(),
		}
	}

	pub fn dark_color(&self) -> Color {
		match self {
			TetrominoKind::I => color::dark_cyan(),
			TetrominoKind::J => color::dark_blue(),
			TetrominoKind::L => color::dark_orange(),
			TetrominoKind::O => color::dark_yellow(),
			TetrominoKind::S => color::dark_green(),
			TetrominoKind::T => color::dark_purple(),
			TetrominoKind::Z => color::dark_red(),
		}
	}
}

mod position_map {
	use super::Position;

	type Map = [Position; 4];

	pub const I: Map = [
		Position::new([(0, 1), (1, 1), (2, 1), (3, 1)]),
		Position::new([(2, 0), (2, 1), (2, 2), (2, 3)]),
		Position::new([(0, 2), (1, 2), (2, 2), (3, 2)]),
		Position::new([(1, 0), (1, 1), (1, 2), (1, 3)]),
	];

	pub const J: Map = [
		Position::new([(0, 0), (0, 1), (1, 1), (2, 1)]),
		Position::new([(2, 0), (1, 0), (1, 1), (1, 2)]),
		Position::new([(2, 2), (2, 1), (1, 1), (0, 1)]),
		Position::new([(0, 2), (1, 2), (1, 1), (1, 0)]),
	];

	pub const L: Map = [
		Position::new([(2, 0), (0, 1), (1, 1), (2, 1)]),
		Position::new([(2, 2), (1, 0), (1, 1), (1, 2)]),
		Position::new([(0, 2), (2, 1), (1, 1), (0, 1)]),
		Position::new([(0, 0), (1, 2), (1, 1), (1, 0)]),
	];

	pub const O: Map = [
		Position::new([(1, 0), (2, 0), (1, 1), (2, 1)]),
		Position::new([(1, 0), (2, 0), (1, 1), (2, 1)]),
		Position::new([(1, 0), (2, 0), (1, 1), (2, 1)]),
		Position::new([(1, 0), (2, 0), (1, 1), (2, 1)]),
	];

	pub const S: Map = [
		Position::new([(1, 0), (2, 0), (0, 1), (1, 1)]),
		Position::new([(2, 1), (2, 2), (1, 0), (1, 1)]),
		Position::new([(1, 2), (0, 2), (2, 1), (1, 1)]),
		Position::new([(0, 1), (0, 0), (1, 2), (1, 1)]),
	];

	pub const T: Map = [
		Position::new([(1, 0), (0, 1), (1, 1), (2, 1)]),
		Position::new([(2, 1), (1, 0), (1, 1), (1, 2)]),
		Position::new([(1, 2), (2, 1), (1, 1), (0, 1)]),
		Position::new([(0, 1), (1, 2), (1, 1), (1, 0)]),
	];

	pub const Z: Map = [
		Position::new([(0, 0), (1, 0), (1, 1), (2, 1)]),
		Position::new([(2, 0), (2, 1), (1, 1), (1, 2)]),
		Position::new([(2, 2), (1, 2), (1, 1), (0, 1)]),
		Position::new([(0, 2), (0, 1), (1, 1), (1, 0)]),
	];
}

mod color {
	use ratatui::style::Color;

	pub fn red() -> Color {
		Color::Rgb(200, 0, 0)
	}

	pub fn orange() -> Color {
		Color::Rgb(255, 165, 0)
	}

	pub fn yellow() -> Color {
		Color::Rgb(255, 255, 0)
	}

	pub fn green() -> Color {
		Color::Rgb(0, 255, 0)
	}

	pub fn cyan() -> Color {
		Color::Rgb(0, 255, 255)
	}

	pub fn blue() -> Color {
		Color::Rgb(5, 50, 255)
	}

	pub fn purple() -> Color {
		Color::Rgb(128, 0, 128)
	}

	pub fn dark_red() -> Color {
		Color::Rgb(139, 0, 0)
	}

	pub fn dark_orange() -> Color {
		Color::Rgb(205, 92, 0)
	}

	pub fn dark_yellow() -> Color {
		Color::Rgb(139, 139, 0)
	}

	pub fn dark_green() -> Color {
		Color::Rgb(0, 68, 27)
	}

	pub fn dark_cyan() -> Color {
		Color::Rgb(0, 139, 139)
	}

	pub fn dark_blue() -> Color {
		Color::Rgb(0, 0, 139)
	}

	pub fn dark_purple() -> Color {
		Color::Rgb(64, 0, 64)
	}
}
