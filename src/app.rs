use anyhow::Result;

use crate::{
	channel::{channel, Event, KeyEvent},
	handler::Handler,
	save::Save,
	state::State,
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: Handler,
	state: State,
	save: Save,
}

impl App {
	pub fn new() -> Result<Self> {
		let mut save = Save::new();

		save.read()?;

		let term = Term::new()?;
		let (state_tx, state_rx) = channel();
		let handler = Handler::new(state_rx);
		let mut state = State::new(state_tx);

		state.read_save(&mut save);

		Ok(Self {
			term,
			handler,
			state,
			save,
		})
	}

	pub async fn run(&mut self) -> Result<()> {
		self.term.init()?;

		self.term.draw(|f| {
			ui(f, &self.state);
		})?;

		while let Some(event) = self.handler.next().await {
			if let Event::Key(KeyEvent::CtrlC) = &event {
				break;
			}

			self.state.handle_event(event);

			self.term.draw(|f| {
				ui(f, &self.state);
			})?;

			if !self.state.running {
				break;
			}
		}

		self.save.write(&self.state)?;

		self.term.exit()?;

		Ok(())
	}
}
