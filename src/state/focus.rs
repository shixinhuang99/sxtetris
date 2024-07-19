#[derive(PartialEq, Eq)]
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
	}

	pub fn push(&mut self, focus: Scene) {
		self.history.push(focus);
	}

	pub fn back(&mut self) {
		self.history.pop();
	}

	pub fn current(&self) -> &Scene {
		self.history.last().expect("should be a least one")
	}

	pub fn contains(&self, scene: Scene) -> bool {
		self.history.contains(&scene)
	}
}
