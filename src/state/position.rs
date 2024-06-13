use crate::consts::{BOARD_X_LEN, BOARD_Y_LEN};

type Point = (usize, usize);

#[derive(Clone)]
pub struct Position {
	pub points: [Point; 4],
}

impl Position {
	pub fn new(points: [Point; 4]) -> Self {
		Self {
			points,
		}
	}

	pub fn update<F: FnMut(&mut Point)>(&mut self, f: F) {
		self.points.iter_mut().for_each(f);
	}

	pub fn is_touched_top(&self) -> bool {
		self.points.iter().any(|p| p.1 == 0)
	}

	pub fn is_touched_bottom(&self) -> bool {
		self.points.iter().any(|p| p.1 == BOARD_Y_LEN - 1)
	}

	pub fn is_touched_left(&self) -> bool {
		self.points.iter().any(|p| p.0 == 0)
	}

	pub fn is_touched_right(&self) -> bool {
		self.points.iter().any(|p| p.0 == BOARD_X_LEN - 1)
	}
}

impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		for i in 0..4 {
			if self.points[i].0 != other.points[i].0
				|| self.points[i].1 != other.points[i].1
			{
				return false;
			}
		}
		true
	}
}
