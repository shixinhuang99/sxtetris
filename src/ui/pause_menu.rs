use ratatui::{layout::Constraint, widgets::Clear, Frame};

use super::{
	list::list,
	utils::{centered_rect, rounded_block},
};
use crate::state::ListState;

pub fn pause_menu(f: &mut Frame, list_state: &ListState) {
	let area =
		centered_rect(f.size(), Constraint::Length(44), Constraint::Length(24));

	let block = rounded_block(Some("PAUSED"));

	let block_inner = block.inner(area);

	f.render_widget(Clear, area);

	f.render_widget(block, area);

	list(f, block_inner, list_state);
}
