use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::{Color, Style},
	text::{Line, Span},
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::{list::list, utils::centered_rect};
use crate::{consts::APP_NAME, state::ListState};

pub fn start_menu(f: &mut Frame, rect: Rect, list_state: &ListState) {
	let outer_area = centered_rect(
		rect,
		Constraint::Percentage(60),
		Constraint::Percentage(60),
	);

	let chunks = Layout::vertical([
		Constraint::Percentage(35),
		Constraint::Percentage(65),
	])
	.areas::<2>(outer_area);

	let title_area = Layout::horizontal([Constraint::Length(64)])
		.flex(Flex::Center)
		.areas::<1>(chunks[0])[0];

	let colors = [
		Color::White,
		Color::White,
		Color::Red,
		Color::Blue,
		Color::Yellow,
		Color::Green,
		Color::Cyan,
		Color::Magenta,
	];

	let spans = APP_NAME
		.to_uppercase()
		.chars()
		.enumerate()
		.map(|(i, ch)| Span::styled(ch.to_string(), Style::new().fg(colors[i])))
		.collect::<Vec<Span>>();

	let title = BigText::builder()
		.pixel_size(PixelSize::Full)
		.lines([Line::default().spans(spans)])
		.build()
		.unwrap();

	f.render_widget(title, title_area);

	list(f, chunks[1], list_state);
}
