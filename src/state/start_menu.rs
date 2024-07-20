use crate::common::{Menu, VecExt};

pub struct StartMenu {
	items: Vec<String>,
	cursor: usize,
}

impl StartMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["PLAY", "SCORES", "SETTING", "HELP", "ABOUT", "QUIT"]
				.into_owned_vec(),
			cursor: 0,
		}
	}
}

impl Menu for StartMenu {
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

pub mod start_menu_idx {
	pub const PLAY: usize = 0;
	pub const SCORES: usize = 1;
	pub const SETTING: usize = 2;
	pub const HELP: usize = 3;
	pub const ABOUT: usize = 4;
	pub const QUIT: usize = 5;
}
