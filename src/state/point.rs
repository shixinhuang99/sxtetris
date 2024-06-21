use crate::consts::{BOARD_COLS, BOARD_ROWS, BOARD_VISIBLE_ROWS};

const MAX_Y: i32 = BOARD_ROWS as i32 - 1;
const MAX_X: i32 = BOARD_COLS as i32 - 1;
const MIN_VISIBLE_Y: i32 = BOARD_VISIBLE_ROWS as i32;

pub type Point = (i32, i32);

#[derive(Clone, PartialEq)]
pub struct Points {
	pub value: [Point; 4],
}

impl Points {
	pub fn new(value: [Point; 4]) -> Self {
		Self {
			value,
		}
	}

	pub fn update<F: FnMut(&mut Point)>(&mut self, f: F) {
		self.value.iter_mut().for_each(f);
	}

	pub fn is_touched_bottom(&self) -> bool {
		self.value.iter().any(|p| p.1 >= MAX_Y)
	}

	pub fn is_touched_left(&self) -> bool {
		self.value.iter().any(|p| p.0 <= 0)
	}

	pub fn is_touched_right(&self) -> bool {
		self.value.iter().any(|p| p.0 >= MAX_X)
	}

	pub fn is_out_of_board(&self) -> bool {
		self.value
			.iter()
			.any(|p| p.0 < 0 || p.0 > MAX_X || p.1 < 0 || p.1 > MAX_Y)
	}

	pub fn is_out_of_visible_arae(&self) -> bool {
		self.value.iter().any(|p| {
			p.0 < 0 || p.0 > MAX_X || p.1 < MIN_VISIBLE_Y || p.1 > MAX_Y
		})
	}

	pub fn usize_points(&self) -> [(usize, usize); 4] {
		let mut points = [(0, 0); 4];

		for (i, p) in self.value.iter().enumerate() {
			points[i].0 = p.0 as usize;
			points[i].1 = p.1 as usize;
		}

		points
	}

	pub fn contains(&self, x: usize, y: usize) -> bool {
		self.value
			.iter()
			.any(|p| p.1 as usize == y && p.0 as usize == x)
	}
}
