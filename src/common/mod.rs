mod point;
mod position;
mod tetromino_kind;

pub use position::{pos, Position};
pub use tetromino_kind::TetrominoKind;

use crate::global::{use_audio, Sound};

pub trait Board {
	fn get_kind(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}

pub trait Reset {
	fn reset(&mut self);
}

pub trait Menu {
	fn cursor_mut(&mut self) -> &mut usize;

	fn cursor(&self) -> usize;

	fn items(&self) -> &[&'static str];

	fn up(&mut self) {
		let end = self.items().len() - 1;
		let cursor = self.cursor_mut();

		if *cursor == 0 {
			*cursor = end;
		} else {
			*cursor -= 1;
		}

		use_audio(|audio| audio.play_sound(Sound::Menu));
	}

	fn down(&mut self) {
		let end = self.items().len() - 1;
		let cursor = self.cursor_mut();

		if *cursor == end {
			*cursor = 0;
		} else {
			*cursor += 1;
		}

		use_audio(|audio| audio.play_sound(Sound::Menu));
	}

	fn reset(&mut self) {
		*self.cursor_mut() = 0;
	}
}
