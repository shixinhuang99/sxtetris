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
}

impl Audio {
	pub fn new() -> Self {
		let inner = AudioInner::new().ok();

		Self {
			inner,
		}
	}

	pub fn stop_all(&self) {
		if let Some(inner) = &self.inner {
			inner.bg_music_sink.stop();
			inner.game_over_sink.stop();
			inner.menu_sink.stop();
			inner.lock_sink.stop();
			inner.clear_sink.stop();
			inner.move_sink.stop();
		}
	}

	pub fn play_bg_music(&self) {
		if let Some(inner) = &self.inner {
			if inner.bg_music_sink.empty() {
				inner
					.bg_music_sink
					.append(inner.bg_music_sound_source.clone());
				inner.bg_music_sink.play();
			}
		}
	}

	pub fn stop_bg_music(&self) {
		if let Some(inner) = &self.inner {
			inner.bg_music_sink.clear();
		}
	}

	pub fn paly_game_over_sound(&self) {
		if let Some(inner) = &self.inner {
			if inner.game_over_sink.empty() {
				inner
					.game_over_sink
					.append(inner.game_over_sound_source.clone());
			}
		}
	}

	pub fn paly_menu_key_sound(&self) {
		if let Some(inner) = &self.inner {
			if inner.menu_sink.empty() {
				inner.menu_sink.append(inner.menu_sound_source.clone());
			}
		}
	}

	pub fn paly_move_sound(&self) {
		if let Some(inner) = &self.inner {
			if inner.move_sink.empty() {
				inner.move_sink.append(inner.move_sound_source.clone());
			}
		}
	}

	pub fn paly_lock_sound(&self) {
		if let Some(inner) = &self.inner {
			if inner.lock_sink.empty() {
				inner.lock_sink.append(inner.lock_sound_source.clone());
			}
		}
	}

	pub fn paly_line_clear_sound(&self) {
		if let Some(inner) = &self.inner {
			if inner.clear_sink.empty() {
				inner.clear_sink.append(inner.clear_sound_source.clone());
			} else {
				inner.clear_sink.clear();
				inner.clear_sink.append(inner.clear_sound_source.clone());
				inner.clear_sink.play();
			}
		}
	}
}

struct AudioInner {
	_stream: OutputStream,
	_handle: OutputStreamHandle,
	bg_music_sound_source: Repeat<Amplify<SoundBuf>>,
	bg_music_sink: Sink,
	menu_sound_source: Buffered<SoundBuf>,
	menu_sink: Sink,
	move_sound_source: Buffered<Amplify<SoundBuf>>,
	move_sink: Sink,
	lock_sound_source: Buffered<SoundBuf>,
	lock_sink: Sink,
	clear_sound_source: Buffered<SoundBuf>,
	clear_sink: Sink,
	game_over_sound_source: Buffered<SoundBuf>,
	game_over_sink: Sink,
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
			bg_music_sink,
			menu_sound_source,
			menu_sink,
			move_sound_source,
			move_sink,
			lock_sound_source,
			lock_sink,
			clear_sound_source,
			clear_sink,
			game_over_sound_source,
			game_over_sink,
		})
	}
}
