use ratatui::{layout::Constraint, widgets::Clear, Frame};

use super::{
	list::list,
	utils::{centered_rect, rounded_block},
};
use crate::state::State;

pub fn popup_menu(f: &mut Frame, state: &State) {
	let area =
		centered_rect(f.size(), Constraint::Length(44), Constraint::Length(24));

	let block = rounded_block(Some("MENU"));

	let block_inner = block.inner(area);

	f.render_widget(Clear, area);

	f.render_widget(block, area);

	list(
		f,
		block_inner,
		["CONTINUE", "NEW GAME", "SCORES", "QUIT"],
		state.popup_menu_selected,
	);
}
