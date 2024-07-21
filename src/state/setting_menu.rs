use crate::{
	common::{Menu, VecExt},
	global::{global_audio, global_setting, Sound},
};

pub struct SettingMenu {
	items: Vec<String>,
	cursor: usize,
}

impl SettingMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["PARTICLE", "MUSIC", "SOUND"].into_owned_vec(),
			cursor: 0,
		}
	}

	pub fn handle_enter(&self) {
		use setting_menu_idx::*;

		let setting = global_setting();

		match self.cursor {
			PARTICLE => setting.switch_particle(),
			MUSIC => setting.switch_music(),
			SOUND => setting.switch_sound(),
			_ => (),
		}

		global_audio(|audio| audio.play_sound(Sound::Menu));
	}
}

impl Menu for SettingMenu {
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
		use setting_menu_idx::*;

		let mut items = self.items.clone();
		let setting = global_setting();

		items[PARTICLE] = particle_text(setting.particle());
		items[MUSIC] = music_text(setting.music());
		items[SOUND] = sound_text(setting.sound());

		items
	}
}

mod setting_menu_idx {
	pub const PARTICLE: usize = 0;
	pub const MUSIC: usize = 1;
	pub const SOUND: usize = 2;
}

fn bool_text(v: bool) -> &'static str {
	if v {
		"ON"
	} else {
		"OFF"
	}
}

fn particle_text(v: bool) -> String {
	format!("{:<10}{:>3}", "PARTICLE: ", bool_text(v))
}

fn music_text(v: bool) -> String {
	format!("{:<10}{:>3}", "MUSIC: ", bool_text(v))
}

fn sound_text(v: bool) -> String {
	format!("{:<10}{:>3}", "SOUND: ", bool_text(v))
}
