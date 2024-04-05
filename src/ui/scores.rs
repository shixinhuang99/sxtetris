use ratatui::{layout::Constraint, text::Line, widgets::Clear, Frame};
use tui_big_text::{BigText, PixelSize};

use super::utils::{centered_rect, rounded_block};
use crate::state::State;

pub fn scores(f: &mut Frame, state: &State) {
	let screen = f.size();

	let area =
		centered_rect(screen, Constraint::Length(58), Constraint::Length(42));

	f.render_widget(Clear, area);

	let block = rounded_block(Some("HIGH SCORES"));

	let block_inner = block.inner(area);

	f.render_widget(block, area);

	let lines = state
		.scores
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
		.collect::<Vec<Line>>();

	let text = BigText::builder()
		.pixel_size(PixelSize::Quadrant)
		.lines(lines)
		.build()
		.unwrap();

	f.render_widget(text, block_inner);
}