use super::TetrominoKind;

pub struct Bag {
	value: [char; 7],
	cursor: usize,
}

impl Bag {
	pub fn new() -> Self {
		let mut this = Self {
			value: ['I', 'J', 'L', 'O', 'S', 'T', 'Z'],
			cursor: 0,
		};

		this.shuffle();

		this
	}

	fn shuffle(&mut self) {
		fastrand::shuffle(self.value.as_mut_slice());
	}

	pub fn next(&mut self) -> TetrominoKind {
		if self.cursor >= self.value.len() {
			self.shuffle();
			self.cursor = 0;
		}
		let ch = self.value[self.cursor];
		self.cursor += 1;
		TetrominoKind::from(ch)
	}
}
