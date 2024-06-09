pub struct ListState {
	pub items: Vec<Item>,
	pub cursor: usize,
}

pub struct Item {
	pub idx: usize,
	pub name: &'static str,
	pub hidden: bool,
}

impl ListState {
	pub fn new(names: &[&'static str]) -> Self {
		let mut items = vec![];

		for (idx, name) in names.iter().enumerate() {
			items.push(Item {
				idx,
				name,
				hidden: false,
			});
		}

		Self {
			items,
			cursor: 0,
		}
	}

	pub fn up(&mut self) {
		loop {
			if self.cursor == 0 {
				self.cursor = self.items.len() - 1;
			} else {
				self.cursor -= 1;
			}

			if !self.items[self.cursor].hidden {
				break;
			}
		}
	}

	pub fn down(&mut self) {
		loop {
			if self.cursor == self.items.len() - 1 {
				self.cursor = 0;
			} else {
				self.cursor += 1;
			}

			if !self.items[self.cursor].hidden {
				break;
			}
		}
	}

	pub fn hide(&mut self, idx: usize) {
		self.items[idx].hidden = true;
		self.down();
	}

	pub fn reset(&mut self) {
		if let Some(idx) = self.items.iter().position(|item| !item.hidden) {
			self.cursor = idx;
		} else {
			self.cursor = 0;
		}
	}
}
