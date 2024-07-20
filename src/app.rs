use anyhow::Result;

use crate::{
	global::{global_audio, init_global_audio},
	handler::{Event, MainHandler},
	save::Save,
	state::State,
	term::Term,
	ui::{loading, ui},
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
		self.term.draw(loading)?;

		init_global_audio();
		self.save.read(&mut self.state);
		self.handler.init_task();

		while let Some(event) = self.handler.recv().await {
			if event == Event::CtrlC {
				break;
			}

			if event == Event::Tick {
				self.state.update_line_clear();
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

		global_audio(|audio| audio.stop_all());
		self.save.write(&self.state);
		self.handler.shutdown().await;
		self.state.handler.shutdown().await;

		self.term.exit()?;

		Ok(())
	}
}
