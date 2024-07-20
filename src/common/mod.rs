mod point;
mod position;
mod tetromino_kind;

pub use position::{pos, Position};
pub use tetromino_kind::TetrominoKind;

use crate::global::{global_audio, Sound};

pub trait Board {
	fn get_kind(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}

pub trait Reset {
	fn reset(&mut self);
}

pub trait Menu {
	fn cursor_mut(&mut self) -> &mut usize;

	fn cursor(&self) -> usize;

	fn end(&self) -> usize;

	fn items(&self) -> Vec<String>;

	fn up(&mut self) {
		let end = self.end();
		let cursor = self.cursor_mut();

		if *cursor == 0 {
			*cursor = end;
		} else {
			*cursor -= 1;
		}

		global_audio(|audio| audio.play_sound(Sound::Menu));
	}

	fn down(&mut self) {
		let end = self.end();
		let cursor = self.cursor_mut();

		if *cursor == end {
			*cursor = 0;
		} else {
			*cursor += 1;
		}

		global_audio(|audio| audio.play_sound(Sound::Menu));
	}

	fn reset(&mut self) {
		*self.cursor_mut() = 0;
	}
}

pub trait VecExt<T> {
	fn into_owned_vec(self) -> Vec<T>;
}

impl VecExt<String> for Vec<&str> {
	fn into_owned_vec(self) -> Vec<String> {
		self.into_iter().map(|s| s.to_string()).collect()
	}
}
