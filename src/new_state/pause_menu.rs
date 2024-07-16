use crate::core::Menu;

pub struct PauseMenu {
	items: [&'static str; 6],
	cursor: usize,
}

impl PauseMenu {
	pub fn new() -> Self {
		Self {
			items: ["RESUME", "NEW GAME", "SCORES", "SETTING", "HELP", "QUIT"],
			cursor: 0,
		}
	}
}

impl Menu for PauseMenu {
	fn cursor_and_end(&mut self) -> (&mut usize, usize) {
		(&mut self.cursor, self.items.len() - 1)
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
