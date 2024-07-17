use crate::common::Menu;

pub struct PauseMenu {
	items: Vec<&'static str>,
	cursor: usize,
}

impl PauseMenu {
	pub fn new() -> Self {
		Self {
			items: vec![
				"RESUME", "NEW GAME", "SCORES", "SETTING", "HELP", "QUIT",
			],
			cursor: 0,
		}
	}
}

impl Menu for PauseMenu {
	fn cursor(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn end(&self) -> usize {
		self.items.len() - 1
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
