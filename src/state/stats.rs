use serde::{Deserialize, Serialize};

use crate::common::Reset;

#[derive(Clone, Deserialize, Serialize)]
pub struct Stats {
	pub level: u32,
	pub score: u32,
	pub lines: u32,
	pub combo: i32,
}

impl Stats {
	pub fn new() -> Self {
		Self {
			level: 1,
			score: 0,
			lines: 0,
			combo: -1,
		}
	}

	pub fn update(&mut self, rows_len: usize) {
		let previous_level = self.level;

		if rows_len > 0 {
			self.lines += rows_len as u32;

			let new_level = self.lines / 10 + 1;

			if new_level > previous_level {
				self.level = new_level;
			}

			let base_score = match rows_len {
				1 => 100,
				2 => 300,
				3 => 500,
				4 => 800,
				_ => 0,
			};
			self.score += base_score * self.level;
			self.combo += 1;
		} else {
			self.combo = -1;
		}
		if self.combo > 0 {
			self.score += 50 * self.combo as u32 * self.level;
		}
	}
}

impl Reset for Stats {
	fn reset(&mut self) {
		*self = Self::new();
	}
}
