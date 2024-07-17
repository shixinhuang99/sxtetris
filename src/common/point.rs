use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Point<T> {
	pub x: T,
	pub y: T,
}

impl Point<i8> {
	pub const fn new(raw_point: (i8, i8)) -> Self {
		Self {
			x: raw_point.0,
			y: raw_point.1,
		}
	}

	pub fn to_usize_point(&self) -> Point<usize> {
		self.clone().into()
	}
}

impl From<Point<i8>> for Point<usize> {
	fn from(value: Point<i8>) -> Self {
		Self {
			x: value.x as usize,
			y: value.y as usize,
		}
	}
}
