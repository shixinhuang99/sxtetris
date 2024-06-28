use ratatui::{
	layout::{Constraint, Layout},
	style::{Style, Stylize},
	text::{Line, Text},
	widgets::{block::Padding, Paragraph},
	Frame,
};

use super::utils::Popup;
use crate::consts::APP_VER;

pub fn about(f: &mut Frame) {
	let popup = Popup::new(48, 14)
		.title("ABOUT")
		.padding(Padding::vertical(3))
		.render(f);

	let chunks = Layout::vertical([Constraint::Length(3); 2])
		.spacing(2)
		.areas::<2>(popup);

	let version = Paragraph::new(Text::from(vec![
		Line::raw("Version"),
		Line::raw(APP_VER),
	]))
	.centered()
	.style(Style::new().white().bold());

	f.render_widget(version, chunks[0]);

	let source = Paragraph::new(Text::from(vec![
		Line::raw("GitHub"),
		Line::raw("https://github.com/shixinhuang99/sxtetris"),
	]))
	.centered()
	.style(Style::new().white().bold());

	f.render_widget(source, chunks[1]);
}
