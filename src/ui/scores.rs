use ratatui::{
	style::{Style, Stylize},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::utils::Popup;
use crate::state::State;

pub fn scores(f: &mut Frame, state: &State) {
	let popup = Popup::new(58, 42).title("HIGH SCORES").render(f);

	let lines: Vec<Line> = state
		.scores
		.to_strings()
		.into_iter()
		.map(Line::raw)
		.collect();

	let text = BigText::builder()
		.pixel_size(PixelSize::Quadrant)
		.lines(lines)
		.style(Style::new().white())
		.build()
		.unwrap();

	f.render_widget(text, popup);
}
