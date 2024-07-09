use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Point {
	pub x: i8,
	pub y: i8,
}

impl Point {
	pub const fn new(raw_point: (i8, i8)) -> Self {
		Self {
			x: raw_point.0,
			y: raw_point.1,
		}
	}
}
