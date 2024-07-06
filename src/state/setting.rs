use super::ListState;

mod setting_menu_idx {
	pub const PARTICLES: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}

use setting_menu_idx::*;

pub struct Setting {
	pub menu: ListState,
	pub show: bool,
	pub particles: bool,
	pub music: bool,
	pub sound: bool,
}

impl Setting {
	pub fn new() -> Self {
		Self {
			menu: ListState::new(&["PARTICLES: ON", "MUSIC: OFF", "SOUND: ON"]),
			show: false,
			particles: true,
			music: false,
			sound: true,
		}
	}

	pub fn handle_enter(&mut self) {
		match self.menu.cursor {
			PARTICLES => {
				self.particles = !self.particles;
				self.menu.items[PARTICLES] = if self.particles {
					"PARTICLES: ON"
				} else {
					"PARTICLES: OFF"
				};
			}
			MUSIC => {
				self.music = !self.music;
				self.menu.items[MUSIC] = if self.music {
					"MUSIC: ON"
				} else {
					"MUSIC: OFF"
				};
			}
			SOUND => {
				self.sound = !self.sound;
				self.menu.items[SOUND] = if self.sound {
					"SOUND: ON"
				} else {
					"SOUND: OFF"
				};
			}
			_ => (),
		}
	}

	fn _serialize(&self) -> String {
		format!(
			"#setting\n{}{}{}\n",
			self.particles as u8, self.music as u8, self.sound as u8
		)
	}

	fn _deserialize(&mut self, source: &str) {
		let chars: Vec<char> = source.chars().collect();
		if chars.len() < 3 {
			return;
		}
		self.particles = chars[0] == '1';
		self.music = chars[1] == '1';
		self.sound = chars[2] == '1';
	}
}
