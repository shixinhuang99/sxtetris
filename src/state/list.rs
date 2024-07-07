pub struct ListState {
	pub items: Vec<String>,
	pub cursor: usize,
}

impl ListState {
	pub fn new(items: &[&str]) -> Self {
		Self {
			items: items.iter().map(|s| s.to_string()).collect(),
			cursor: 0,
		}
	}

	pub fn up(&mut self) {
		if self.cursor == 0 {
			self.cursor = self.items.len() - 1;
		} else {
			self.cursor -= 1;
		}
	}

	pub fn down(&mut self) {
		if self.cursor == self.items.len() - 1 {
			self.cursor = 0;
		} else {
			self.cursor += 1;
		}
	}

	pub fn reset(&mut self) {
		self.cursor = 0;
	}
}
