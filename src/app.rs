use std::rc::Rc;

use anyhow::Result;

use crate::{
	audio::Audio,
	handler::{GameEvent, MainHandler},
	save::Save,
	state::State,
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: MainHandler,
	state: State,
	save: Save,
	audio: Rc<Audio>,
}

impl App {
	pub fn new() -> Result<Self> {
		let mut save = Save::new();

		save.read()?;

		let term = Term::new()?;
		let handler = MainHandler::new();
		let audio = Rc::new(Audio::new());
		let mut state = State::new(handler.create_sub_handler(), audio.clone());

		state.read_save(&save);

		Ok(Self {
			term,
			handler,
			state,
			save,
			audio,
		})
	}

	pub async fn run(&mut self) -> Result<()> {
		self.term.init()?;

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

		self.audio.stop_all();

		self.save.write(&self.state)?;

		self.term.exit()?;

		Ok(())
	}
}
