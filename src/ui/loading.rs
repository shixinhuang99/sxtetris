use ratatui::{layout::Constraint, text::Line, Frame};
use tui_big_text::{BigText, PixelSize};

use super::utils::centered_rect;

pub fn loading(f: &mut Frame) {
	let area =
		centered_rect(f.size(), Constraint::Length(80), Constraint::Length(8));

	let title = BigText::builder()
		.pixel_size(PixelSize::Full)
		.lines([Line::raw("LOADING...")])
		.build()
		.unwrap();

	f.render_widget(title, area);
}
