use ratatui::{
	style::{Style, Stylize},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::utils::Popup;
use crate::state::scores::Scores;

pub fn scores(f: &mut Frame, scores: &Scores) {
	let popup = Popup::new(58, 42).title("HIGH SCORES").render(f);

	let lines: Vec<Line> = scores
		.iter()
		.enumerate()
		.map(|(i, score)| {
			let s = if i >= 9 {
				format!("{}.{:>11}", i + 1, score)
			} else {
				format!("{}.{:>12}", i + 1, score)
			};
			Line::raw(s)
		})
		.collect();

	let text = BigText::builder()
		.pixel_size(PixelSize::Quadrant)
		.lines(lines)
		.style(Style::new().white())
		.build()
		.unwrap();

	f.render_widget(text, popup);
}
