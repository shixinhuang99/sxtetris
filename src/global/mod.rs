mod audio;
pub mod setting;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub use audio::{Sound, AUDIO};
pub use setting::global_setting;

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
