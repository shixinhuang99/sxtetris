pub struct ListState {
	pub items: Vec<&'static str>,
	pub cursor: usize,
}

impl ListState {
	pub fn new(items: &[&'static str]) -> Self {
		Self {
			items: Vec::from(items),
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
