mod audio;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use audio::Audio;
pub use audio::Sound;

static PAUSED: AtomicBool = AtomicBool::new(false);
static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn is_paused() -> bool {
	PAUSED.load(Relaxed)
}

pub fn set_paused(v: bool) {
	PAUSED.store(v, Relaxed);
}

pub fn is_locked() -> bool {
	LOCKED.load(Relaxed)
}

pub fn set_locked(v: bool) {
	LOCKED.store(v, Relaxed);
}

thread_local! {
	static AUDIO: Audio = Audio::new();
}

pub fn use_audio<F: FnOnce(&Audio)>(f: F) {
	AUDIO.with(f);
}
