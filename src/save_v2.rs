use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;

use crate::{consts::APP_NAME, save::Save as SaveV1, state::State};

const SAVE_FILE: &str = "save_v2.txt";

pub struct Save {
	file: PathBuf,
	save_map: HashMap<String, String>,
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
			fs::create_dir_all(&dir)?;
		}

		Ok(Self {
			file,
			save_map: HashMap::new(),
		})
	}

	pub fn read(&mut self, state: &mut State) -> Result<()> {
		if !self.file.exists() {
			if let Ok(mut save_v1) = SaveV1::try_new() {
				if save_v1.read().is_ok() {
					state.read_save_v1(&save_v1);
				}
			}
			fs::write(&self.file, "")?;
		} else if self.read_to_map().is_ok() {
			if self.save_map.is_empty() {
				return Ok(());
			}
			let saveables = state.get_saveables_for_read();
			for saveable in saveables {
				if let Some(content) = self.save_map.get(saveable.get_key()) {
					saveable.read_content(content);
				}
			}
			if self.save_map.contains_key("board") {
				state.count_down = 3;
			}
		}

		Ok(())
	}

	pub fn write(&self, saveables: Vec<&dyn Saveable>) -> Result<()> {
		let mut items: Vec<String> = Vec::new();
		let mut keep_last_game = true;

		for saveable in saveables {
			let key = saveable.get_key();
			if key == "board" {
				keep_last_game = false;
			}
			items.push(format!("{}\n{}", key, saveable.get_content()));
		}

		if keep_last_game && self.save_map.contains_key("board") {
			for (k, v) in &self.save_map {
				if k == "setting" || k == "scores" {
					continue;
				}
				items.push(format!("{}\n{}", k, v));
			}
		}

		fs::write(&self.file, items.join("\n"))?;

		Ok(())
	}

	fn read_to_map(&mut self) -> Result<()> {
		let content = fs::read_to_string(&self.file)?;
		let lines: Vec<&str> = content.lines().collect();

		for chunk in lines.chunks_exact(2) {
			self.save_map
				.insert(chunk[0].to_string(), chunk[1].to_string());
		}

		Ok(())
	}
}

pub trait Saveable {
	fn get_key(&self) -> &'static str;
	fn get_content(&self) -> String;
	fn read_content(&mut self, content: &str);
}
