use anyhow::Result;

use crate::{
	channel::{channel, Event, KeyEvent},
	handler::Handler,
	state::State,
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: Handler,
	state: State,
}

impl App {
	pub fn new() -> Result<Self> {
		let term = Term::new()?;

		let (state_tx, state_rx) = channel();

		let handler = Handler::new(state_rx);

		let state = State::new(state_tx);

		Ok(Self {
			term,
			handler,
			state,
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

		self.term.exit()?;

		Ok(())
	}
}
