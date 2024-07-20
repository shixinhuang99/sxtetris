use crate::common::{Menu, VecExt};

pub struct GameOverMenu {
	items: Vec<String>,
	cursor: usize,
	pub new_score: Option<String>,
}

impl GameOverMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["NEW GAME", "SCORES", "QUIT"].into_owned_vec(),
			cursor: 0,
			new_score: None,
		}
	}

	pub fn set_new_score(&mut self, score: u32, idx: Option<usize>) {
		if let Some(i) = idx {
			self.new_score = Some(format!("{}.{:>11}", i + 1, score));
		} else {
			self.new_score = None;
		}
	}
}

impl Menu for GameOverMenu {
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
		self.items.clone()
	}
}

pub mod game_over_menu_idx {
	pub const NEW_GAME: usize = 0;
	pub const SCORES: usize = 1;
	pub const QUIT: usize = 2;
}
