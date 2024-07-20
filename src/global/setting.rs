use std::sync::{
	atomic::{AtomicBool, Ordering::Relaxed},
	OnceLock,
};

use super::AUDIO;

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
			AUDIO.with(|audio| audio.pause_music());
		} else {
			AUDIO.with(|audio| audio.resume_music());
		}
	}

	pub fn switch_sound(&self) {
		let previous = self.sound.load(Relaxed);
		self.sound.store(!previous, Relaxed);
		if previous {
			AUDIO.with(|audio| audio.stop_sound());
		}
	}
}
