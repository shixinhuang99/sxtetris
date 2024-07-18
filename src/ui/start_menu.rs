use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	text::{Line, Span},
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::{menu::menu, utils::centered_rect};
use crate::{consts::APP_NAME, new_state::start_menu::StartMenu};

pub fn start_menu(f: &mut Frame, rect: Rect, start_menu: &StartMenu) {
	let outer_area = centered_rect(
		rect,
		Constraint::Percentage(60),
		Constraint::Percentage(100),
	);

	let chunks = Layout::vertical([
		Constraint::Percentage(30),
		Constraint::Percentage(70),
	])
	.areas::<2>(outer_area);

	let title_area =
		centered_rect(chunks[0], Constraint::Length(64), Constraint::Length(8));

	let spans: Vec<Span> = APP_NAME
		.to_uppercase()
		.chars()
		.enumerate()
		.map(|(i, ch)| Span::styled(ch.to_string(), Style::new().fg(COLORS[i])))
		.collect();

	let title = BigText::builder()
		.pixel_size(PixelSize::Full)
		.lines([Line::default().spans(spans)])
		.build()
		.unwrap();

	f.render_widget(title, title_area);

	menu(f, chunks[1], start_menu);
}

const COLORS: [Color; 8] = [
	Color::White,
	Color::White,
	Color::Red,
	Color::Blue,
	Color::Yellow,
	Color::Green,
	Color::Cyan,
	Color::Magenta,
];
