use crate::common::Menu;

pub struct SettingMenu {
	items: Vec<&'static str>,
	cursor: usize,
}

impl SettingMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["PARTICLES", "MUSIC", "SOUND"],
			cursor: 0,
		}
	}
}

impl Menu for SettingMenu {
	fn cursor(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn end(&self) -> usize {
		self.items.len() - 1
	}
}

pub mod setting_menu_idx {
	pub const PARTICLES: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}
