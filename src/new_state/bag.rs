use crate::common::TetrominoKind;

pub struct Bag {
	kinds: Vec<TetrominoKind>,
	cursor: usize,
	last: Option<TetrominoKind>,
}

impl Bag {
	pub fn new() -> Self {
		Self {
			kinds: vec![
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
		}
	}

	pub fn shuffle(&mut self) {
		self.cursor = 0;
		fastrand::shuffle(self.kinds.as_mut_slice());
		if self.last.is_some_and(|last| last == self.kinds[0]) {
			self.kinds.swap(0, 1);
		}
	}

	pub fn next(&mut self) -> TetrominoKind {
		if self.cursor >= self.kinds.len() {
			self.shuffle();
		}
		let kind = self.kinds[self.cursor];
		self.cursor += 1;
		self.last = Some(kind);

		kind
	}
}
