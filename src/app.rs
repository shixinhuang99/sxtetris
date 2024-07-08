use anyhow::Result;

use crate::{
	common::save::{LastGame, Save},
	handler::{GameEvent, MainHandler},
	state::{Screen, State},
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: MainHandler,
	state: State,
	save: Option<Save>,
}

impl App {
	pub fn new() -> Result<Self> {
		let term = Term::new()?;
		let handler = MainHandler::new();
		let state = State::new(handler.create_sub_handler());
		let save = Save::try_new().ok();

		Ok(Self {
			term,
			handler,
			state,
			save,
		})
	}

	pub async fn run(&mut self) -> Result<()> {
		self.term.init()?;

		self.read_save();

		self.state.check_setting();

		while let Some(event) = self.handler.recv().await {
			if event == GameEvent::CtrlC {
				break;
			}

			if event == GameEvent::Tick {
				self.state.update_clear_rows_progress();
				self.term.draw(|f| {
					ui(f, &mut self.state);
				})?;
				continue;
			}

			self.state.receive_event(event);

			if !self.state.running {
				break;
			}
		}

		self.state.audio.stop_all();

		self.wirte_save();

		self.term.exit()?;

		Ok(())
	}

	fn read_save(&mut self) {
		let Some(save) = &mut self.save else {
			return;
		};
		if save.try_read().is_ok() {
			self.state.scores.clone_from(&save.content.scores);
			self.state.setting.clone_from(&save.content.settting);
			self.state.setting.update_menu();
			let Some(last_game) = &save.content.last_game else {
				return;
			};
			self.state.count_down = 3;
			self.state.board.board.clone_from(&last_game.board.board);
			self.state.bag.clone_from(&last_game.bag);
			self.state.stats.clone_from(&last_game.stats);
			self.state.active_tm.clone_from(&last_game.active_tm);
			self.state.preview_tm.clone_from(&last_game.preview_tm);
		}
	}

	fn wirte_save(&mut self) {
		let Some(save) = &mut self.save else {
			return;
		};
		save.content.scores.clone_from(&self.state.scores);
		save.content.settting.clone_from(&self.state.setting);
		if self.state.screen == Screen::Game {
			if self.state.is_game_over {
				save.content.last_game = None;
			} else {
				save.content.last_game = Some(LastGame {
					board: self.state.board.clone(),
					bag: self.state.bag.clone(),
					stats: self.state.stats.clone(),
					active_tm: self.state.active_tm.clone(),
					preview_tm: self.state.preview_tm.clone(),
				});
			}
		}
		let _ = save.try_write();
	}
}
