use crate::common::{Menu, VecExt};

pub struct PauseMenu {
	items: Vec<String>,
	cursor: usize,
}

impl PauseMenu {
	pub fn new() -> Self {
		Self {
			items: vec![
				"RESUME", "NEW GAME", "SCORES", "SETTING", "HELP", "QUIT",
			]
			.into_owned_vec(),
			cursor: 0,
		}
	}
}

impl Menu for PauseMenu {
	fn cursor_mut(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn cursor(&self) -> usize {
		self.cursor
	}

	fn end(&self) -> usize {
		self.items.len() - 1
	}

	fn items(&self) -> Vec<String> {
		self.items.clone()
	}
}

pub mod pause_menu_idx {
	pub const RESUME: usize = 0;
	pub const NEW_GAME: usize = 1;
	pub const SCORES: usize = 2;
	pub const SETTING: usize = 3;
	pub const HELP: usize = 4;
	pub const QUIT: usize = 5;
}
