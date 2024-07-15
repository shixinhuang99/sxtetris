use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Point {
	pub x: i8,
	pub y: i8,
}

pub struct BoardPoint {
	pub x: usize,
	pub y: usize,
}

impl Point {
	pub const fn new(raw_point: (i8, i8)) -> Self {
		Self {
			x: raw_point.0,
			y: raw_point.1,
		}
	}

	pub fn to_board_point(&self) -> BoardPoint {
		self.clone().into()
	}
}

impl From<Point> for BoardPoint {
	fn from(value: Point) -> Self {
		Self {
			x: value.x as usize,
			y: value.y as usize,
		}
	}
}
