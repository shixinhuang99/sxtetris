use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::{Style, Stylize},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::common::Menu;

pub fn menu(f: &mut Frame, rect: Rect, menuable: &dyn Menu) {
	let cursor = menuable.cursor();
	let items = menuable.items();

	let v_chunks = Layout::vertical(vec![Constraint::Length(4); items.len()])
		.spacing(2)
		.split(rect);

	for (i, item) in items.iter().enumerate() {
		let title = BigText::builder()
			.pixel_size(PixelSize::Quadrant)
			.lines([Line::raw(item)])
			.style(
				if i == cursor {
					Style::new().light_yellow()
				} else {
					Style::new().white()
				},
			)
			.build()
			.unwrap();

		let title_area =
			Layout::horizontal([Constraint::Length(item.len() as u16 * 4)])
				.flex(Flex::Center)
				.areas::<1>(v_chunks[i])[0];

		f.render_widget(title, title_area);
	}
}
