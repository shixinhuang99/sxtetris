use anyhow::Result;

use crate::{
	handler::{GameEvent, MainHandler},
	save_v2::Save,
	state::State,
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

		if let Some(save) = &mut self.save {
			let _ = save.read(&mut self.state);
		}

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

		if let Some(save) = &self.save {
			let _ = save.write(self.state.get_saveables_for_write());
		}

		self.term.exit()?;

		Ok(())
	}
}
