use crate::consts::{BOARD_X_LEN, BOARD_Y_LEN};

const MAX_Y: i32 = BOARD_Y_LEN as i32 - 1;
const MAX_X: i32 = BOARD_X_LEN as i32 - 1;

pub type Point = (i32, i32);

#[derive(Clone)]
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

	pub fn is_touched_top(&self) -> bool {
		self.value.iter().any(|p| p.1 <= 0)
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

	pub fn is_out_of_border(&self) -> bool {
		self.value
			.iter()
			.any(|p| p.0 < 0 || p.0 > MAX_X || p.1 < 0 || p.1 > MAX_Y)
	}
}

impl PartialEq for Points {
	fn eq(&self, other: &Self) -> bool {
		for i in 0..4 {
			if self.value[i].0 != other.value[i].0
				|| self.value[i].1 != other.value[i].1
			{
				return false;
			}
		}
		true
	}
}
