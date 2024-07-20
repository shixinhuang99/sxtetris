use ratatui::Frame;

use super::{menu::menu, utils::Popup};
use crate::state::pause_menu::PauseMenu;

pub fn pause_menu(f: &mut Frame, pause_menu: &PauseMenu) {
	let popup = Popup::new(44, 36).title("PAUSED").render(f);

	menu(f, popup, pause_menu);
}
