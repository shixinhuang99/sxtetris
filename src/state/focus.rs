use crate::global::{Sound, AUDIO};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Scene {
	StartMenu,
	Game,
	PauseMenu,
	GameOverMenu,
	SettingMenu,
	Scores,
	Help,
	About,
}

pub struct Focus {
	history: Vec<Scene>,
}

impl Focus {
	pub fn new() -> Self {
		Self {
			history: vec![Scene::StartMenu],
		}
	}

	pub fn to(&mut self, focus: Scene) {
		self.history.clear();
		self.history.push(focus);

		AUDIO.with(|audio| audio.play_sound(Sound::Menu));
	}

	pub fn push(&mut self, focus: Scene) {
		self.history.push(focus);

		if focus != Scene::GameOverMenu {
			AUDIO.with(|audio| audio.play_sound(Sound::Menu));
		}
	}

	pub fn back(&mut self) {
		self.history.pop();

		AUDIO.with(|audio| audio.play_sound(Sound::Menu));
	}

	pub fn current(&self) -> &Scene {
		self.history.last().expect("should be a least one")
	}

	pub fn contains(&self, scene: Scene) -> bool {
		self.history.contains(&scene)
	}
}
