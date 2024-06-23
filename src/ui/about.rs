use ratatui::{
	layout::{Constraint, Layout},
	style::{Style, Stylize},
	text::{Line, Text},
	widgets::{block::Padding, Clear, Paragraph},
	Frame,
};

use super::utils::{centered_rect, rounded_block};
use crate::consts::APP_VER;

pub fn about(f: &mut Frame) {
	let area =
		centered_rect(f.size(), Constraint::Length(50), Constraint::Length(14));

	f.render_widget(Clear, area);

	let block = rounded_block(Some("ABOUT")).padding(Padding::vertical(3));

	let block_inner = block.inner(area);

	f.render_widget(block, area);

	let chunks = Layout::vertical([Constraint::Length(3); 2])
		.spacing(2)
		.areas::<2>(block_inner);

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
