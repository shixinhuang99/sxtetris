use ratatui::{widgets::Padding, Frame};

use super::{menu::menu, utils::Popup};
use crate::state::setting_menu::SettingMenu;

pub fn setting_menu(f: &mut Frame, setting_menu: &SettingMenu) {
	let popup = Popup::new(60, 22)
		.title("SETTING")
		.padding(Padding::vertical(2))
		.render(f);

	menu(f, popup, setting_menu);
}
