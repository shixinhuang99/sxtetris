use crate::common::Menu;

pub struct SettingMenu {
	items: Vec<&'static str>,
	cursor: usize,
}

impl SettingMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["PARTICLE", "MUSIC", "SOUND"],
			cursor: 0,
		}
	}

	pub fn handle_enter(&self) {
		use setting_menu_idx::*;

		match self.cursor {
			PARTICLE => (),
			MUSIC => (),
			SOUND => (),
			_ => (),
		}
	}
}

impl Menu for SettingMenu {
	fn cursor_mut(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn cursor(&self) -> usize {
		self.cursor
	}

	fn items(&self) -> &[&'static str] {
		&self.items
	}
}

mod setting_menu_idx {
	pub const PARTICLE: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}
