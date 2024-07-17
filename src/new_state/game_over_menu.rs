use crate::common::Menu;

pub struct GameOverMenu {
	items: Vec<&'static str>,
	cursor: usize,
	new_score: Option<String>,
}

impl GameOverMenu {
	pub fn new() -> Self {
		Self {
			items: vec!["NEW GAME", "SCORES", "QUIT"],
			cursor: 0,
			new_score: None,
		}
	}

	pub fn set_new_score(&mut self, score_and_idx: Option<(u32, usize)>) {
		if let Some((score, idx)) = score_and_idx {
			self.new_score = Some(format!("{}.{:>11}", idx + 1, score));
		} else {
			self.new_score = None;
		}
	}
}

impl Menu for GameOverMenu {
	fn cursor(&mut self) -> &mut usize {
		&mut self.cursor
	}

	fn end(&self) -> usize {
		self.items.len() - 1
	}
}

pub mod game_over_menu_idx {
	pub const NEW_GAME: usize = 0;
	pub const SCORES: usize = 1;
	pub const QUIT: usize = 2;
}
