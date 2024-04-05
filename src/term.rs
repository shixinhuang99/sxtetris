use std::{
	io::{stderr, Stderr},
	panic,
};

use anyhow::Result;
use crossterm::{
	execute,
	terminal::{
		disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
		LeaveAlternateScreen, SetTitle,
	},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};

use crate::consts::APP_NAME;

#[derive(Debug)]
pub struct Term {
	terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Term {
	pub fn new() -> Result<Self> {
		let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

		Ok(Self {
			terminal,
		})
	}

	pub fn init(&mut self) -> Result<()> {
		enable_raw_mode()?;
		execute!(stderr(), SetTitle(APP_NAME), EnterAlternateScreen)?;

		let panic_hook = panic::take_hook();
		panic::set_hook(Box::new(move |panic| {
			reset().expect("failed to reset the terminal");
			panic_hook(panic);
		}));

		self.terminal.hide_cursor()?;
		self.terminal.clear()?;
		Ok(())
	}

	pub fn draw<F: FnOnce(&mut Frame)>(&mut self, f: F) -> Result<()> {
		self.terminal.draw(f)?;

		Ok(())
	}

	pub fn exit(&mut self) -> Result<()> {
		reset()?;
		self.terminal.show_cursor()?;

		Ok(())
	}
}

fn reset() -> Result<()> {
	disable_raw_mode()?;
	crossterm::execute!(stderr(), LeaveAlternateScreen)?;

	Ok(())
}
