use std::io::Cursor;

use anyhow::Result;
use rodio::{
	source::{Amplify, Buffered, Repeat},
	Decoder, OutputStream, OutputStreamHandle, Sink, Source,
};

static BG_MUSIC_BYTES: &[u8] = include_bytes!("assets/bg_music.mp3");
static GAME_OVER_SOUND_BYTES: &[u8] = include_bytes!("assets/game_over.wav");
static MENU_SOUND_BYTES: &[u8] = include_bytes!("assets/menu.wav");
static MOVE_SOUND_BYTES: &[u8] = include_bytes!("assets/move.wav");
static LOCK_SOUND_BYTES: &[u8] = include_bytes!("assets/lock.wav");
static CLEAR_SOUND_BYTES: &[u8] = include_bytes!("assets/clear.mp3");

type SoundBuf = Decoder<Cursor<&'static [u8]>>;

pub struct Audio {
	inner: Option<AudioInner>,
	music_enable: bool,
	sound_enable: bool,
}

impl Audio {
	pub fn new() -> Self {
		let inner = AudioInner::new().ok();

		Self {
			inner,
			music_enable: false,
			sound_enable: true,
		}
	}

	pub fn stop_all(&self) {
		if let Some(inner) = &self.inner {
			inner.bg_music_sound.stop();
			inner.game_over_sound.stop();
			inner.menu_sound.stop();
			inner.lock_sound.stop();
			inner.clear_sound.stop();
			inner.move_sound.stop();
		}
	}

	pub fn disable_sound_effects(&mut self) {
		self.sound_enable = false;
		if let Some(inner) = &self.inner {
			inner.game_over_sound.clear();
			inner.menu_sound.clear();
			inner.lock_sound.clear();
			inner.clear_sound.clear();
			inner.move_sound.clear();
		}
	}

	pub fn enable_sound_effects(&mut self) {
		self.sound_enable = true;
	}

	pub fn disable_music(&mut self) {
		self.music_enable = false;
		if let Some(inner) = &self.inner {
			inner.bg_music_sound.pause();
		}
	}

	pub fn enable_music(&mut self) {
		self.music_enable = true;
	}

	pub fn play_bg_music(&self) {
		if !self.music_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.bg_music_sound.empty() {
				inner
					.bg_music_sound
					.append(inner.bg_music_sound_source.clone());
			}
			inner.bg_music_sound.play();
		}
	}

	pub fn stop_bg_music(&self) {
		if let Some(inner) = &self.inner {
			inner.bg_music_sound.clear();
		}
	}

	pub fn paly_game_over_sound(&self) {
		if !self.sound_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.game_over_sound.empty() {
				inner
					.game_over_sound
					.append(inner.game_over_sound_source.clone());
			}
			inner.game_over_sound.play();
		}
	}

	pub fn paly_menu_key_sound(&self) {
		if !self.sound_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.menu_sound.empty() {
				inner.menu_sound.append(inner.menu_sound_source.clone());
			}
			inner.menu_sound.play();
		}
	}

	pub fn paly_move_sound(&self) {
		if !self.sound_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.move_sound.empty() {
				inner.move_sound.append(inner.move_sound_source.clone());
			}
			inner.move_sound.play();
		}
	}

	pub fn paly_lock_sound(&self) {
		if !self.sound_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.lock_sound.empty() {
				inner.lock_sound.append(inner.lock_sound_source.clone());
			}
			inner.lock_sound.play();
		}
	}

	pub fn paly_line_clear_sound(&self) {
		if !self.sound_enable {
			return;
		}
		if let Some(inner) = &self.inner {
			if inner.clear_sound.empty() {
				inner.clear_sound.append(inner.clear_sound_source.clone());
			} else {
				inner.clear_sound.clear();
				inner.clear_sound.append(inner.clear_sound_source.clone());
			}
			inner.clear_sound.play();
		}
	}
}

struct AudioInner {
	_stream: OutputStream,
	_handle: OutputStreamHandle,
	bg_music_sound_source: Repeat<Amplify<SoundBuf>>,
	bg_music_sound: Sink,
	menu_sound_source: Buffered<SoundBuf>,
	menu_sound: Sink,
	move_sound_source: Buffered<Amplify<SoundBuf>>,
	move_sound: Sink,
	lock_sound_source: Buffered<SoundBuf>,
	lock_sound: Sink,
	clear_sound_source: Buffered<SoundBuf>,
	clear_sound: Sink,
	game_over_sound_source: Buffered<SoundBuf>,
	game_over_sound: Sink,
}

impl AudioInner {
	fn new() -> Result<Self> {
		let (_stream, _handle) = OutputStream::try_default()?;

		let bg_music_sound_source =
			Decoder::new_mp3(Cursor::new(BG_MUSIC_BYTES))?
				.amplify(0.7)
				.repeat_infinite();
		let bg_music_sink = Sink::try_new(&_handle)?;

		let menu_sound_source =
			Decoder::new_wav(Cursor::new(MENU_SOUND_BYTES))?.buffered();
		let menu_sink = Sink::try_new(&_handle)?;

		let move_sound_source =
			Decoder::new_wav(Cursor::new(MOVE_SOUND_BYTES))?
				.amplify(3.0)
				.buffered();
		let move_sink = Sink::try_new(&_handle)?;

		let lock_sound_source =
			Decoder::new_wav(Cursor::new(LOCK_SOUND_BYTES))?.buffered();
		let lock_sink = Sink::try_new(&_handle)?;

		let clear_sound_source =
			Decoder::new_mp3(Cursor::new(CLEAR_SOUND_BYTES))?.buffered();
		let clear_sink = Sink::try_new(&_handle)?;

		let game_over_sound_source =
			Decoder::new_wav(Cursor::new(GAME_OVER_SOUND_BYTES))?.buffered();
		let game_over_sink = Sink::try_new(&_handle)?;

		Ok(Self {
			_stream,
			_handle,
			bg_music_sound_source,
			bg_music_sound: bg_music_sink,
			menu_sound_source,
			menu_sound: menu_sink,
			move_sound_source,
			move_sound: move_sink,
			lock_sound_source,
			lock_sound: lock_sink,
			clear_sound_source,
			clear_sound: clear_sink,
			game_over_sound_source,
			game_over_sound: game_over_sink,
		})
	}
}
