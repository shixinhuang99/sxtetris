use std::slice::Iter;

use serde::{Deserialize, Serialize};

use super::point::Point;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Position([Point; 4]);

impl Position {
	pub const fn new(raw_points: [(i8, i8); 4]) -> Self {
		Self([
			Point::new(raw_points[0]),
			Point::new(raw_points[1]),
			Point::new(raw_points[2]),
			Point::new(raw_points[3]),
		])
	}

	pub fn update<F: FnMut(&mut Point)>(&mut self, f: F) {
		self.0.iter_mut().for_each(f);
	}

	pub fn iter(&self) -> Iter<Point> {
		self.0.iter()
	}
}
