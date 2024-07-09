use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use super::point::Point;

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum TetrominoType {
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

impl TetrominoType {
	pub fn init_points(&self, idx: usize) -> [Point; 4] {
		match self {
			TetrominoType::I => rotate_map::I[idx],
			TetrominoType::J => rotate_map::J[idx],
			TetrominoType::L => rotate_map::L[idx],
			TetrominoType::O => rotate_map::O[idx],
			TetrominoType::S => rotate_map::S[idx],
			TetrominoType::T => rotate_map::T[idx],
			TetrominoType::Z => rotate_map::Z[idx],
			_ => [(0, 0); 4],
		}
	}

	pub fn is_none_or_ghost(&self) -> bool {
		matches!(self, TetrominoType::None | TetrominoType::Ghost)
	}

	pub fn color(&self) -> Color {
		match self {
			TetrominoType::I => color::cyan(),
			TetrominoType::O => color::yellow(),
			TetrominoType::T => color::purple(),
			TetrominoType::L => color::orange(),
			TetrominoType::J => color::blue(),
			TetrominoType::S => color::green(),
			TetrominoType::Z => color::red(),
			TetrominoType::None => Color::DarkGray,
			TetrominoType::Ghost => Color::Gray,
		}
	}

	pub fn dark_color(&self) -> Color {
		match self {
			TetrominoType::I => color::dark_cyan(),
			TetrominoType::O => color::dark_yellow(),
			TetrominoType::T => color::dark_purple(),
			TetrominoType::L => color::dark_orange(),
			TetrominoType::J => color::dark_blue(),
			TetrominoType::S => color::dark_green(),
			TetrominoType::Z => color::dark_red(),
			TetrominoType::None => Color::DarkGray,
			TetrominoType::Ghost => Color::Gray,
		}
	}
}

impl From<char> for TetrominoType {
	fn from(value: char) -> Self {
		match value {
			'I' => TetrominoType::I,
			'J' => TetrominoType::J,
			'L' => TetrominoType::L,
			'O' => TetrominoType::O,
			'S' => TetrominoType::S,
			'T' => TetrominoType::T,
			'Z' => TetrominoType::Z,
			_ => TetrominoType::None,
		}
	}
}

impl From<TetrominoType> for char {
	fn from(value: TetrominoType) -> Self {
		match value {
			TetrominoType::I => 'I',
			TetrominoType::J => 'J',
			TetrominoType::L => 'L',
			TetrominoType::O => 'O',
			TetrominoType::S => 'S',
			TetrominoType::T => 'T',
			TetrominoType::Z => 'Z',
			TetrominoType::None | TetrominoType::Ghost => '-',
		}
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

	pub const O: Map = [
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
		[(1, 0), (2, 0), (1, 1), (2, 1)],
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
}

mod color {
	use ratatui::style::Color;

	pub const fn red() -> Color {
		Color::Rgb(200, 0, 0)
	}

	pub const fn orange() -> Color {
		Color::Rgb(255, 165, 0)
	}

	pub const fn yellow() -> Color {
		Color::Rgb(255, 255, 0)
	}

	pub const fn green() -> Color {
		Color::Rgb(0, 255, 0)
	}

	pub const fn cyan() -> Color {
		Color::Rgb(0, 255, 255)
	}

	pub const fn blue() -> Color {
		Color::Rgb(5, 50, 255)
	}

	pub const fn purple() -> Color {
		Color::Rgb(128, 0, 128)
	}

	pub const fn dark_red() -> Color {
		Color::Rgb(139, 0, 0)
	}

	pub const fn dark_orange() -> Color {
		Color::Rgb(205, 92, 0)
	}

	pub const fn dark_yellow() -> Color {
		Color::Rgb(139, 139, 0)
	}

	pub const fn dark_green() -> Color {
		Color::Rgb(0, 68, 27)
	}

	pub const fn dark_cyan() -> Color {
		Color::Rgb(0, 139, 139)
	}

	pub const fn dark_blue() -> Color {
		Color::Rgb(0, 0, 139)
	}

	pub const fn dark_purple() -> Color {
		Color::Rgb(64, 0, 64)
	}
}
