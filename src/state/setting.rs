use super::ListState;

mod setting_menu_idx {
	pub const PARTICLES: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}

use setting_menu_idx::*;

use crate::save_v2::Saveable;

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
				self.menu.items[self.menu.cursor] =
					setting_text("PARTICLES", self.particles);
			}
			MUSIC => {
				self.music = !self.music;
				self.menu.items[self.menu.cursor] =
					setting_text("MUSIC", self.music);
			}
			SOUND => {
				self.sound = !self.sound;
				self.menu.items[self.menu.cursor] =
					setting_text("SOUND", self.sound);
			}
			_ => (),
		}
	}
}

impl Saveable for Setting {
	fn get_key(&self) -> &'static str {
		"setting"
	}

	fn get_content(&self) -> String {
		format!(
			"{}{}{}",
			self.particles as u8, self.music as u8, self.sound as u8
		)
	}

	fn read_content(&mut self, content: &str) {
		let chars: Vec<char> = content.chars().collect();
		if chars.len() != 3 {
			return;
		}
		self.particles = chars[0] == '1';
		self.menu.items[0] = setting_text("PARTICLES", self.particles);
		self.music = chars[1] == '1';
		self.menu.items[1] = setting_text("MUSIC", self.music);
		self.sound = chars[2] == '1';
		self.menu.items[2] = setting_text("SOUND", self.sound);
	}
}

fn setting_text(k: &str, v: bool) -> String {
	format!(
		"{}: {}",
		k,
		if v {
			"ON"
		} else {
			"OFF"
		}
	)
}
