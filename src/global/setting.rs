use std::sync::{
	atomic::{AtomicBool, Ordering::Relaxed},
	OnceLock,
};

use serde::{Deserialize, Serialize};

use super::global_audio;

static SETTING: OnceLock<Setting> = OnceLock::new();

pub fn global_setting() -> &'static Setting {
	SETTING.get_or_init(Setting::new)
}

pub struct Setting {
	particle: AtomicBool,
	music: AtomicBool,
	sound: AtomicBool,
}

impl Setting {
	pub fn new() -> Self {
		Self {
			particle: AtomicBool::new(false),
			music: AtomicBool::new(false),
			sound: AtomicBool::new(false),
		}
	}

	pub fn particle(&self) -> bool {
		self.particle.load(Relaxed)
	}

	pub fn music(&self) -> bool {
		self.music.load(Relaxed)
	}

	pub fn sound(&self) -> bool {
		self.sound.load(Relaxed)
	}

	pub fn switch_particle(&self) {
		let previous = self.particle.load(Relaxed);
		self.particle.store(!previous, Relaxed);
	}

	pub fn switch_music(&self) {
		let previous = self.music.load(Relaxed);
		self.music.store(!previous, Relaxed);
		if previous {
			global_audio(|audio| audio.pause_music());
		} else {
			global_audio(|audio| audio.resume_music());
		}
	}

	pub fn switch_sound(&self) {
		let previous = self.sound.load(Relaxed);
		self.sound.store(!previous, Relaxed);
		if previous {
			global_audio(|audio| audio.stop_sound());
		}
	}

	pub fn to_save_content(&self) -> SettingSave {
		SettingSave {
			particle: self.particle(),
			music: self.music(),
			sound: self.sound(),
		}
	}

	pub fn read_from_save(&self, content: &SettingSave) {
		self.particle.store(content.particle, Relaxed);
		self.music.store(content.music, Relaxed);
		self.sound.store(content.sound, Relaxed);
		if content.sound {
			global_audio(|audio| audio.stop_sound());
		}
	}
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct SettingSave {
	particle: bool,
	music: bool,
	sound: bool,
}
