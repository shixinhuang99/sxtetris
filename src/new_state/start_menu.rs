use crate::core::Menu;

pub struct StartMenu {
	items: [&'static str; 6],
	cursor: usize,
}

impl StartMenu {
	pub fn new() -> Self {
		Self {
			items: ["PLAY", "SCORES", "SETTING", "HELP", "ABOUT", "QUIT"],
			cursor: 0,
		}
	}
}

impl Menu for StartMenu {
	fn cursor_and_end(&mut self) -> (&mut usize, usize) {
		(&mut self.cursor, self.items.len() - 1)
	}
}

pub mod start_menu_idx {
	pub const PLAY: usize = 0;
	pub const SCORES: usize = 1;
	pub const SETTING: usize = 2;
	pub const HELP: usize = 3;
	pub const ABOUT: usize = 4;
	pub const QUIT: usize = 5;
}
