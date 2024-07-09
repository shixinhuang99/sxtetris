use ratatui::{
	layout::{Constraint, Flex, Layout},
	style::{Color, Style},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::{
	list::list,
	utils::{rounded_block, Popup},
};
use crate::state::State;

pub fn game_over_menu(f: &mut Frame, state: &mut State) {
	let new_score = state.scores.get_new_score();
	let (width_offest, height_offest) = if new_score.is_some() {
		(8, 10)
	} else {
		(0, 0)
	};

	let popup = Popup::new(48 + width_offest, 26 + height_offest).render(f);

	let mut constraints = vec![Constraint::Length(4), Constraint::Length(16)];

	if new_score.is_some() {
		constraints.insert(1, Constraint::Length(6));
	}

	let chunk = Layout::vertical(constraints).spacing(3).split(popup);

	if let Some(score) = new_score {
		let new_score_block = rounded_block().title("NEW SCORE");
		let new_score_block_inner = new_score_block.inner(chunk[1]);
		let score = BigText::builder()
			.pixel_size(PixelSize::Quadrant)
			.lines([Line::raw(score)])
			.style(Style::new().fg(Color::Green))
			.build()
			.unwrap();

		f.render_widget(new_score_block, chunk[1]);
		f.render_widget(score, new_score_block_inner);
	}

	let title = BigText::builder()
		.pixel_size(PixelSize::Quadrant)
		.lines([Line::raw("GAME OVER")])
		.style(Style::new().fg(Color::Red))
		.build()
		.unwrap();

	let title_area = Layout::horizontal([Constraint::Length(36)])
		.flex(Flex::Center)
		.areas::<1>(chunk[0])[0];

	f.render_widget(title, title_area);

	list(
		f,
		if chunk.len() == 3 {
			chunk[2]
		} else {
			chunk[1]
		},
		&state.game_over_menu,
	);
}
