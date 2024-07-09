use serde::{Deserialize, Serialize};

use super::ListState;

#[derive(Clone, Deserialize, Serialize)]
pub struct Setting {
	#[serde(skip, default = "default_menu")]
	pub menu: ListState,
	#[serde(skip)]
	pub show: bool,
	pub particles: bool,
	pub music: bool,
	pub sound: bool,
}

impl Setting {
	pub fn new() -> Self {
		Self {
			menu: default_menu(),
			show: false,
			particles: false,
			music: false,
			sound: false,
		}
	}

	pub fn handle_enter(&mut self) {
		use setting_menu_idx::*;

		let cursor = self.menu.cursor;
		match cursor {
			PARTICLES => {
				self.particles = !self.particles;
				self.menu.items[cursor] = text("PARTICLES", self.particles);
			}
			MUSIC => {
				self.music = !self.music;
				self.menu.items[cursor] = text("MUSIC", self.music);
			}
			SOUND => {
				self.sound = !self.sound;
				self.menu.items[cursor] = text("SOUND", self.sound);
			}
			_ => (),
		}
	}

	pub fn update_menu(&mut self) {
		use setting_menu_idx::*;

		self.menu.items[PARTICLES] = text("PARTICLES", self.particles);
		self.menu.items[MUSIC] = text("MUSIC", self.music);
		self.menu.items[SOUND] = text("SOUND", self.sound);
	}
}

fn text(k: &str, v: bool) -> String {
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

fn default_menu() -> ListState {
	ListState::new(&["PARTICLES: OFF", "MUSIC: OFF", "SOUND: OFF"])
}

mod setting_menu_idx {
	pub const PARTICLES: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}
