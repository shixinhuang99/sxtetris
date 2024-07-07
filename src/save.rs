use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;

use crate::consts::APP_NAME;

const SAVE_FILE: &str = "save.txt";

pub struct Save {
	file: PathBuf,
	pub scores: Vec<u32>,
	pub last_game: Option<LastGame>,
}

pub struct LastGame {
	pub board: String,
	pub bag: String,
	pub active_tm: String,
	pub preview_tm: String,
	pub level: u32,
	pub score: u32,
	pub lines: u32,
	pub combo: i32,
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

		Ok(Self {
			file,
			scores: vec![0; 10],
			last_game: None,
		})
	}

	pub fn read(&mut self) -> Result<()> {
		use line_map::*;

		if !self.file.exists() {
			anyhow::bail!("no save_v1 file");
		}

		let content = fs::read_to_string(&self.file)?;
		let content_lines: Vec<&str> = content.lines().collect();

		if content_lines.is_empty()
			|| (content_lines.len() != 11 && content_lines.len() != 27)
		{
			return Ok(());
		}

		let mut scores = Vec::new();
		let mut board = String::new();
		let mut bag = String::new();
		let mut active_tm = String::new();
		let mut preview_tm = String::new();
		let mut level = 1;
		let mut score = 0;
		let mut lines = 0;
		let mut combo = -1;

		let mut last_game_read = false;

		for (i, line) in content_lines.into_iter().enumerate() {
			let num = i + 1;

			if SCORES.contains(&num) {
				scores.push(line.parse::<u32>().unwrap_or(0));
			} else {
				match num {
					BOARD => board.push_str(line),
					BAG => bag.push_str(line),
					ACTIVE_TM => active_tm.push_str(line),
					PREVIEW_TM => preview_tm.push_str(line),
					LEVEL => level = line.parse::<u32>().unwrap_or(1),
					SCORE => score = line.parse::<u32>().unwrap_or(0),
					LINES => lines = line.parse::<u32>().unwrap_or(0),
					COMBO => {
						combo = line.parse::<i32>().unwrap_or(-1);
						last_game_read = true;
					}
					_ => (),
				}
			}
		}

		self.scores = scores;

		if last_game_read {
			self.last_game = Some(LastGame {
				board,
				bag,
				active_tm,
				preview_tm,
				level,
				score,
				lines,
				combo,
			});
		}

		Ok(())
	}
}

mod line_map {
	pub const SCORES: std::ops::Range<usize> = 2..12;

	pub const BOARD: usize = 13;

	pub const BAG: usize = 15;

	pub const ACTIVE_TM: usize = 17;

	pub const PREVIEW_TM: usize = 19;

	pub const LEVEL: usize = 21;

	pub const SCORE: usize = 23;

	pub const LINES: usize = 25;

	pub const COMBO: usize = 27;
}
