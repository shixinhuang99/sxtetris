use std::{
	ops::{AddAssign, SubAssign},
	slice::Iter,
};

use serde::{Deserialize, Serialize};

use super::point::{BoardPoint, Point};
use crate::consts::{MAIN_BOARD_BUFFER_ROWS, MAIN_BOARD_COLS, MAIN_BOARD_ROWS};

const MAX_Y: i8 = MAIN_BOARD_ROWS as i8 - 1;
const MAX_X: i8 = MAIN_BOARD_COLS as i8 - 1;
const MIN_Y: i8 = MAIN_BOARD_BUFFER_ROWS as i8;

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

	pub fn iter(&self) -> Iter<Point> {
		self.0.iter()
	}

	pub fn update<F: FnMut(&mut Point)>(&mut self, f: F) {
		self.0.iter_mut().for_each(f);
	}

	pub fn is_touch_bottom(&self) -> bool {
		self.0.iter().any(|p| p.y >= MAX_Y)
	}

	pub fn is_touch_left(&self) -> bool {
		self.0.iter().any(|p| p.x <= 0)
	}

	pub fn is_touch_right(&self) -> bool {
		self.0.iter().any(|p| p.x >= MAX_X)
	}

	pub fn is_outside_the_board(&self) -> bool {
		self.0
			.iter()
			.any(|p| p.x < 0 || p.x > MAX_X || p.y < 0 || p.y > MAX_Y)
	}

	pub fn is_outside_the_visible(&self) -> bool {
		self.0
			.iter()
			.any(|p| p.x < 0 || p.x > MAX_X || p.y < MIN_Y || p.y > MAX_Y)
	}

	pub fn to_board_points(&self) -> Vec<BoardPoint> {
		self.0.iter().map(|p| p.to_board_point()).collect()
	}
}

impl SubAssign for Position {
	fn sub_assign(&mut self, rhs: Self) {
		for i in 0..self.0.len() {
			self.0[i].x = self.0[i].x - rhs.0[i].x;
			self.0[i].y = self.0[i].y - rhs.0[i].y;
		}
	}
}

impl AddAssign for Position {
	fn add_assign(&mut self, rhs: Self) {
		for i in 0..self.0.len() {
			self.0[i].x = self.0[i].x + rhs.0[i].x;
			self.0[i].y = self.0[i].y + rhs.0[i].y;
		}
	}
}
