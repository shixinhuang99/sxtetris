use crate::save_v2::Saveable;

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

	pub fn reset(&mut self) {
		self.level = 1;
		self.score = 0;
		self.lines = 0;
		self.combo = -1;
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

impl Saveable for Stats {
	fn get_key(&self) -> &'static str {
		"stats"
	}

	fn get_content(&self) -> String {
		let items = [
			self.level.to_string(),
			self.score.to_string(),
			self.lines.to_string(),
			self.combo.to_string(),
		];

		items.join(" ")
	}

	fn read_content(&mut self, content: &str) {
		let chunks: Vec<&str> = content.split_ascii_whitespace().collect();

		if chunks.len() != 4 {
			return;
		}

		self.level = chunks[0].parse::<u32>().unwrap_or(1);
		self.score = chunks[1].parse::<u32>().unwrap_or(0);
		self.lines = chunks[2].parse::<u32>().unwrap_or(0);
		self.combo = chunks[3].parse::<i32>().unwrap_or(-1);
	}
}
