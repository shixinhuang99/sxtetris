pub mod point;
pub mod position;
pub mod tetromino_kind;

use tetromino_kind::TetrominoKind;

pub trait Board {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}

pub trait Reset {
	fn reset(&mut self);
}

pub trait Menu {
	fn cursor_and_end(&mut self) -> (&mut usize, usize);

	fn up(&mut self) {
		let (cursor, end) = self.cursor_and_end();
		if *cursor == 0 {
			*cursor = end;
		} else {
			*cursor -= 1;
		}
	}

	fn down(&mut self) {
		let (cursor, end) = self.cursor_and_end();
		if *cursor == end {
			*cursor = 0;
		} else {
			*cursor += 1;
		}
	}
}
