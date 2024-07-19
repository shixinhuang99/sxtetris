use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
	consts::APP_NAME,
	state::{
		bag::Bag, focus::Scene, main_board::MainBoard, next_board::NextBoard,
		scores::Scores, stats::Stats, tetromino::Tetromino, State,
	},
};

const SAVE_FILE: &str = "save.json";

pub struct Save {
	inner: Option<SaveInner>,
}

struct SaveInner {
	file: PathBuf,
	pub content: SaveContent,
}

#[derive(Deserialize, Serialize)]
struct SaveContent {
	scores: Scores,
	last_game: Option<LastGame>,
}

#[derive(Deserialize, Serialize)]
struct LastGame {
	board: MainBoard,
	next_board: NextBoard,
	bag: Bag,
	stats: Stats,
	alive_tetromino: Tetromino,
}

impl Save {
	pub fn new() -> Self {
		Self {
			inner: SaveInner::try_new().ok(),
		}
	}

	pub fn read_to_state(&mut self, state: &mut State) {
		if let Some(inner) = &mut self.inner {
			inner.read_to_state(state);
		}
	}

	pub fn write_state_to_save(&mut self, state: &State) {
		if let Some(inner) = &mut self.inner {
			inner.write_state_to_save(state);
		}
	}
}

impl SaveInner {
	fn try_new() -> Result<Self> {
		let dir = if cfg!(feature = "_dev") {
			PathBuf::from("./")
		} else {
			ProjectDirs::from("", "", APP_NAME)
				.ok_or(anyhow::anyhow!("failed to read save directory"))?
				.config_dir()
				.to_path_buf()
		};

		let file = dir.join(SAVE_FILE);

		if !dir.exists() {
			fs::create_dir_all(dir)?;
		}

		if !file.exists() {
			fs::write(&file, "")?;
		}

		Ok(Self {
			file,
			content: SaveContent {
				scores: Scores::new(),
				last_game: None,
			},
		})
	}

	fn try_read(&mut self) -> Result<()> {
		let content_str = fs::read_to_string(&self.file)?;
		self.content = serde_json::from_str(&content_str)?;

		Ok(())
	}

	fn try_write(&self) -> Result<()> {
		let content_str = serde_json::to_string(&self.content)?;
		fs::write(&self.file, content_str)?;

		Ok(())
	}

	fn read_to_state(&mut self, state: &mut State) {
		if self.try_read().is_ok() {
			state.scores.clone_from(&self.content.scores);
			let Some(last_game) = self.content.last_game.take() else {
				return;
			};
			state.count_down = 3;
			state.board.replace(last_game.board);
			state.next_board.clone_from(&last_game.next_board);
			state.bag.clone_from(&last_game.bag);
			state.stats.clone_from(&last_game.stats);
			state.alive_tetromino.clone_from(&last_game.alive_tetromino);
			state.alive_tetromino.set_board(state.board.clone());
		}
	}

	fn write_state_to_save(&mut self, state: &State) {
		self.content.scores = state.scores.clone();
		self.content.last_game =
			if *state.focus.current() != Scene::GameOverMenu {
				Some(LastGame {
					board: state.board.borrow().clone(),
					next_board: state.next_board.clone(),
					bag: state.bag.clone(),
					alive_tetromino: state.alive_tetromino.clone(),
					stats: state.stats.clone(),
				})
			} else {
				None
			};

		let _ = self.try_write();
	}
}
