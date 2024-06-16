use super::TetrominoKind;

pub struct Bag {
	value: [TetrominoKind; 7],
	cursor: usize,
}

impl Bag {
	pub fn new() -> Self {
		let mut bag = Self {
			value: [
				TetrominoKind::I,
				TetrominoKind::J,
				TetrominoKind::L,
				TetrominoKind::O,
				TetrominoKind::S,
				TetrominoKind::T,
				TetrominoKind::Z,
			],
			cursor: 0,
		};

		bag.shuffle();

		bag
	}

	fn shuffle(&mut self) {
		fastrand::shuffle(self.value.as_mut_slice());
	}

	pub fn next(&mut self) -> TetrominoKind {
		if self.cursor >= self.value.len() {
			self.shuffle();
			self.cursor = 0;
		}
		let tm_kind = self.value[self.cursor];
		self.cursor += 1;

		tm_kind
	}
}
