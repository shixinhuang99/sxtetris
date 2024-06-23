use super::TetrominoKind;

pub struct Bag {
	kinds: [TetrominoKind; 7],
	cursor: usize,
	last: Option<TetrominoKind>,
	count: u8,
}

impl Bag {
	pub fn new() -> Self {
		let mut bag = Self {
			kinds: [
				TetrominoKind::I,
				TetrominoKind::J,
				TetrominoKind::L,
				TetrominoKind::O,
				TetrominoKind::S,
				TetrominoKind::T,
				TetrominoKind::Z,
			],
			cursor: 0,
			last: None,
			count: 0,
		};

		bag.shuffle();

		bag
	}

	fn shuffle(&mut self) {
		fastrand::shuffle(self.kinds.as_mut_slice());
		if self.last.is_some_and(|last| last == self.kinds[0]) && self.count < 3
		{
			self.count += 1;
			return self.shuffle();
		}
		self.count = 0;
	}

	pub fn next(&mut self) -> TetrominoKind {
		if self.cursor >= self.kinds.len() {
			self.shuffle();
			self.cursor = 0;
		}
		let tm_kind = self.kinds[self.cursor];
		self.cursor += 1;
		self.last = Some(tm_kind);

		tm_kind
	}

	pub fn reset(&mut self) {
		self.shuffle();
		self.cursor = 0;
		self.last = None;
		self.count = 0;
	}

	pub fn serialize(&self) -> String {
		let mut content = String::from("#bag\n");

		for kind in &self.kinds {
			content.push(char::from(*kind));
		}

		content.push_str(&format!(" {}\n", self.cursor));

		content
	}

	pub fn deserialize(&mut self, source: &str) {
		let chunks: Vec<&str> = source.split_ascii_whitespace().collect();

		if chunks.len() != 2 {
			return;
		}

		for (i, ch) in chunks[0].chars().enumerate() {
			self.kinds[i] = TetrominoKind::from(ch);
		}

		self.cursor = chunks[1].parse::<usize>().unwrap();
	}
}
