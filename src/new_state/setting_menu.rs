use crate::core::Menu;

pub struct SettingMenu {
	items: [&'static str; 3],
	cursor: usize,
}

impl SettingMenu {
	pub fn new() -> Self {
		Self {
			items: ["PARTICLES", "MUSIC", "SOUND"],
			cursor: 0,
		}
	}
}

impl Menu for SettingMenu {
	fn cursor_and_end(&mut self) -> (&mut usize, usize) {
		(&mut self.cursor, self.items.len() - 1)
	}
}

pub mod setting_menu_idx {
	pub const PARTICLES: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}
