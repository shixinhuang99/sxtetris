use std::{cell::OnceCell, collections::HashMap, io::Cursor};

use anyhow::Result;
use rodio::{
	source::{Amplify, Buffered, Repeat},
	Decoder, OutputStream, OutputStreamHandle, Sink, Source,
};

use super::{global_setting, is_played};

type SoundSource = Buffered<Amplify<Decoder<Cursor<&'static [u8]>>>>;
type MusicSource = Repeat<Amplify<Decoder<Cursor<&'static [u8]>>>>;

thread_local! {
	static AUDIO: OnceCell<Audio> = const { OnceCell::new() };
}

pub fn global_audio<F: FnOnce(&Audio)>(f: F) {
	AUDIO.with(|cell| {
		if let Some(audio) = cell.get() {
			f(audio);
		}
	})
}

pub fn init_global_audio() {
	AUDIO.with(|cell| {
		let _ = cell.set(Audio::new());
	})
}

pub struct Audio {
	inner: Option<AudioInner>,
}

impl Audio {
	pub fn new() -> Self {
		Self {
			inner: AudioInner::new().ok(),
		}
	}

	pub fn stop_all(&self) {
		self.stop_music();
		self.stop_sound();
	}

	pub fn stop_sound(&self) {
		if let Some(inner) = &self.inner {
			inner.short_sound_sink.stop();
			inner.line_clear_sound_sink.stop();
			inner.game_over_sound_sink.stop();
		}
	}

	pub fn pause_music(&self) {
		if let Some(inner) = &self.inner {
			inner.music_sink.pause();
		}
	}

	pub fn resume_music(&self) {
		if let Some(inner) = &self.inner {
			if is_played() && inner.music_sink.empty() {
				inner.music_sink.append(inner.music.clone());
			}
			inner.music_sink.play();
		}
	}

	pub fn stop_music(&self) {
		if let Some(inner) = &self.inner {
			inner.music_sink.stop();
		}
	}

	pub fn play_music(&self) {
		if !global_setting().music() {
			return;
		}
		if let Some(inner) = &self.inner {
			if !inner.music_sink.empty() {
				inner.music_sink.clear();
			}
			inner.music_sink.append(inner.music.clone());
			inner.music_sink.play();
		}
	}

	pub fn play_sound(&self, sound: Sound) {
		if !global_setting().sound() {
			return;
		}
		if let Some(inner) = &self.inner {
			if let Some(source) = inner.sound_map.get(&sound.into()) {
				let source = source.clone();
				match sound {
					Sound::Clear => {
						inner.line_clear_sound_sink.append(source);
					}
					Sound::GameOver => {
						inner.game_over_sound_sink.append(source);
					}
					_ => inner.short_sound_sink.append(source),
				}
			}
		}
	}
}

struct AudioInner {
	_stream: OutputStream,
	_handle: OutputStreamHandle,
	music: MusicSource,
	music_sink: Sink,
	sound_map: HashMap<u8, SoundSource>,
	short_sound_sink: Sink,
	line_clear_sound_sink: Sink,
	game_over_sound_sink: Sink,
}

impl AudioInner {
	fn new() -> Result<Self> {
		let music_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/bg_music.mp3"));
		let menu_sound_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/menu.wav"));
		let move_sound_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/move.wav"));
		let lock_sound_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/lock.wav"));
		let clear_sound_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/clear.mp3"));
		let game_over_sound_buf: Cursor<&[u8]> =
			Cursor::new(include_bytes!("assets/game_over.wav"));

		let (_stream, _handle) = OutputStream::try_default()?;

		let music = Decoder::new_mp3(music_buf)?.amplify(0.7).repeat_infinite();
		let music_sink = Sink::try_new(&_handle)?;

		let mut sound_map = HashMap::new();

		let menu_sound_source =
			Decoder::new_wav(menu_sound_buf)?.amplify(1.0).buffered();

		let move_sound_source =
			Decoder::new_wav(move_sound_buf)?.amplify(3.0).buffered();

		let lock_sound_source =
			Decoder::new_wav(lock_sound_buf)?.amplify(1.0).buffered();

		let clear_sound_source =
			Decoder::new_mp3(clear_sound_buf)?.amplify(1.0).buffered();

		let game_over_sound_source = Decoder::new_wav(game_over_sound_buf)?
			.amplify(1.0)
			.buffered();

		sound_map.insert(Sound::Menu.into(), menu_sound_source);
		sound_map.insert(Sound::Move.into(), move_sound_source);
		sound_map.insert(Sound::Lock.into(), lock_sound_source);
		sound_map.insert(Sound::Clear.into(), clear_sound_source);
		sound_map.insert(Sound::GameOver.into(), game_over_sound_source);

		let short_sound_sink = Sink::try_new(&_handle)?;
		let line_clear_sound_sink = Sink::try_new(&_handle)?;
		let game_over_sound_sink = Sink::try_new(&_handle)?;

		Ok(Self {
			_stream,
			_handle,
			music,
			music_sink,
			sound_map,
			short_sound_sink,
			line_clear_sound_sink,
			game_over_sound_sink,
		})
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Sound {
	Menu,
	Move,
	Lock,
	Clear,
	GameOver,
}

impl From<Sound> for u8 {
	fn from(value: Sound) -> Self {
		match value {
			Sound::Menu => 0,
			Sound::Move => 1,
			Sound::Lock => 2,
			Sound::Clear => 3,
			Sound::GameOver => 4,
		}
	}
}
