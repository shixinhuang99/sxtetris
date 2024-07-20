use std::{
	array::IntoIter,
	ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use super::point::Point;
use crate::consts::{MAIN_BOARD_BUFFER_ROWS, MAIN_BOARD_COLS, MAIN_BOARD_ROWS};

const MAX_Y: i8 = MAIN_BOARD_ROWS as i8 - 1;
const MAX_X: i8 = MAIN_BOARD_COLS as i8 - 1;
const MIN_Y: i8 = MAIN_BOARD_BUFFER_ROWS as i8;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Position([Point<i8>; 4]);

impl Position {
	pub const fn new(raw_points: [(i8, i8); 4]) -> Self {
		Self([
			Point::new(raw_points[0]),
			Point::new(raw_points[1]),
			Point::new(raw_points[2]),
			Point::new(raw_points[3]),
		])
	}

	pub fn into_iter(self) -> IntoIter<Point<i8>, 4> {
		self.0.into_iter()
	}

	pub fn update<F: FnMut(&mut Point<i8>)>(&mut self, f: F) {
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

	pub fn to_usize_points(&self) -> Vec<Point<usize>> {
		self.0.iter().map(|p| p.to_usize_point()).collect()
	}

	pub fn bottom_point(&self) -> &Point<i8> {
		self.0.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap()
	}

	pub fn contains(&self, x: usize, y: usize) -> bool {
		self.0
			.iter()
			.any(|p| p.x as usize == x && p.y as usize == y)
	}
}

pub const fn pos(raw_points: [(i8, i8); 4]) -> Position {
	Position::new(raw_points)
}

impl Default for Position {
	fn default() -> Self {
		Self::new([(0, 0); 4])
	}
}

impl Sub for Position {
	type Output = Position;

	fn sub(mut self, rhs: Self) -> Self::Output {
		for i in 0..self.0.len() {
			self.0[i].x -= rhs.0[i].x;
			self.0[i].y -= rhs.0[i].y;
		}
		self
	}
}

impl Add for Position {
	type Output = Position;

	fn add(mut self, rhs: Self) -> Self::Output {
		for i in 0..self.0.len() {
			self.0[i].x += rhs.0[i].x;
			self.0[i].y += rhs.0[i].y;
		}
		self
	}
}

impl Add<Point<i8>> for Position {
	type Output = Position;

	fn add(mut self, rhs: Point<i8>) -> Self::Output {
		self.update(|p| {
			p.x += rhs.x;
			p.y += rhs.y;
		});
		self
	}
}
