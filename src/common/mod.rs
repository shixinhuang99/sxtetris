mod point;
mod position;
mod tetromino_kind;

pub use position::Position;
pub use tetromino_kind::TetrominoKind;

pub trait Board {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}

pub trait Reset {
	fn reset(&mut self);
}

pub trait Menu {
	fn cursor(&mut self) -> &mut usize;

	fn end(&self) -> usize;

	fn up(&mut self) {
		let end = self.end();
		let cursor = self.cursor();

		if *cursor == 0 {
			*cursor = end;
		} else {
			*cursor -= 1;
		}
	}

	fn down(&mut self) {
		let end = self.end();
		let cursor = self.cursor();

		if *cursor == end {
			*cursor = 0;
		} else {
			*cursor += 1;
		}
	}
}
