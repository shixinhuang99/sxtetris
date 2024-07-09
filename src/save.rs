use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
	consts::APP_NAME,
	state::{Bag, BoardState, Scores, Setting, Stats, Tetromino},
};

const SAVE_FILE: &str = "save.json";

pub struct Save {
	file: PathBuf,
	pub content: SaveContent,
}

#[derive(Deserialize, Serialize)]
pub struct SaveContent {
	pub settting: Setting,
	pub scores: Scores,
	pub last_game: Option<LastGame>,
}

#[derive(Deserialize, Serialize)]
pub struct LastGame {
	pub board: BoardState,
	pub bag: Bag,
	pub stats: Stats,
	pub active_tm: Tetromino,
	pub preview_tm: Tetromino,
}

impl Save {
	pub fn try_new() -> Result<Self> {
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
				settting: Setting::new(),
				scores: Scores::new(),
				last_game: None,
			},
		})
	}

	pub fn try_read(&mut self) -> Result<()> {
		let content_str = fs::read_to_string(&self.file)?;
		self.content = serde_json::from_str(&content_str)?;

		Ok(())
	}

	pub fn try_write(&self) -> Result<()> {
		let content_str = serde_json::to_string(&self.content)?;
		fs::write(&self.file, content_str)?;

		Ok(())
	}
}
