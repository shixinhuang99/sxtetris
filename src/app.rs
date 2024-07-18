use anyhow::Result;

use crate::{
	handler::{Event, MainHandler},
	new_state::State,
	save::Save,
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: MainHandler,
	state: State,
	save: Save,
}

impl App {
	pub fn new() -> Result<Self> {
		let term = Term::new()?;
		let handler = MainHandler::new();
		let state = State::new(handler.create_sub_handler());
		let save = Save::new();

		Ok(Self {
			term,
			handler,
			state,
			save,
		})
	}

	pub async fn run(&mut self) -> Result<()> {
		self.term.init()?;

		self.save.read_to_state(&mut self.state);

		while let Some(event) = self.handler.recv().await {
			if event == Event::CtrlC {
				break;
			}

			if event == Event::Tick {
				// self.state.update_clear_rows_progress();
				self.term.draw(|f| {
					ui(f, &mut self.state);
				})?;
				continue;
			}

			self.state.handle_event(event);

			if !self.state.running {
				break;
			}
		}

		// self.state.audio.stop_all();

		self.save.write_state_to_save(&self.state);

		self.term.exit()?;

		Ok(())
	}
}
