use ratatui::{widgets::Padding, Frame};

use super::{list::list, utils::Popup};
use crate::state::ListState;

pub fn setting_menu(f: &mut Frame, list_state: &ListState) {
	let popup = Popup::new(60, 22)
		.title("SETTING")
		.padding(Padding::vertical(2))
		.render(f);

	list(f, popup, list_state);
}
