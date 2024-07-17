use crate::common::Menu;

pub struct StartMenu {
	items: Vec<&'static str>,
	cursor: usize,
}

impl StartMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["PLAY", "SCORES", "SETTING", "HELP", "ABOUT", "QUIT"],
			cursor: 0,
		}
	}
}

impl Menu for StartMenu {
	fn cursor(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn end(&self) -> usize {
		self.items.len() - 1
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
