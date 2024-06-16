use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::{Color, Style},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::state::ListState;

pub fn list(f: &mut Frame, rect: Rect, list_state: &ListState) {
	let vertical_chunks =
		Layout::vertical(vec![Constraint::Length(4); list_state.items.len()])
			.spacing(2)
			.split(rect);

	for (i, item) in list_state.items.iter().enumerate() {
		let mut title_builder = BigText::builder();

		title_builder
			.pixel_size(PixelSize::Quadrant)
			.lines([Line::raw(*item)]);

		if i == list_state.cursor {
			title_builder.style(Style::new().fg(Color::Yellow));
		}

		let title = title_builder.build().unwrap();

		let title_area =
			Layout::horizontal([Constraint::Length(item.len() as u16 * 4)])
				.flex(Flex::Center)
				.areas::<1>(vertical_chunks[i])[0];

		f.render_widget(title, title_area);
	}
}
