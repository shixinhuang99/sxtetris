use ratatui::Frame;

use super::{list::list, utils::Popup};
use crate::state::ListState;

pub fn pause_menu(f: &mut Frame, list_state: &ListState) {
	let popup = Popup::new(44, 36).title("PAUSED").render(f);

	list(f, popup, list_state);
}
