use super::TetrominoType;
use crate::save_v2::Saveable;

pub struct Bag {
	tm_types: [TetrominoType; 7],
	cursor: usize,
	last: Option<TetrominoType>,
	count: u8,
}

impl Bag {
	pub fn new() -> Self {
		let mut bag = Self {
			tm_types: [
				TetrominoType::I,
				TetrominoType::J,
				TetrominoType::L,
				TetrominoType::O,
				TetrominoType::S,
				TetrominoType::T,
				TetrominoType::Z,
			],
			cursor: 0,
			last: None,
			count: 0,
		};

		bag.shuffle();

		bag
	}

	fn shuffle(&mut self) {
		self.cursor = 0;
		fastrand::shuffle(self.tm_types.as_mut_slice());
		if self.last.is_some_and(|last| last == self.tm_types[0])
			&& self.count < 1
		{
			self.count += 1;
			return self.shuffle();
		}
		self.count = 0;
	}

	pub fn next(&mut self) -> TetrominoType {
		if self.cursor >= self.tm_types.len() {
			self.shuffle();
		}
		let tm_type = self.tm_types[self.cursor];
		self.cursor += 1;
		self.last = Some(tm_type);

		tm_type
	}

	pub fn reset(&mut self) {
		self.shuffle();
		self.last = None;
	}

	pub fn read_save_v1(&mut self, source: &str) {
		let chunks: Vec<&str> = source.split_ascii_whitespace().collect();

		if chunks.len() != 2 {
			return;
		}

		for (i, ch) in chunks[0].chars().enumerate() {
			self.tm_types[i] = TetrominoType::from(ch);
		}

		self.cursor = chunks[1].parse::<usize>().unwrap_or(0);
	}
}

impl Saveable for Bag {
	fn get_key(&self) -> &'static str {
		"bag"
	}

	fn get_content(&self) -> String {
		let mut content = String::new();

		for tm_type in &self.tm_types {
			content.push(char::from(*tm_type));
		}

		content.push_str(&format!(" {}", self.cursor));

		content
	}

	fn read_content(&mut self, content: &str) {
		self.read_save_v1(content);
	}
}
