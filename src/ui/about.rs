use ratatui::{
	layout::Constraint,
	style::{Style, Stylize},
	text::{Line, Text},
	widgets::{block::Padding, Clear, Paragraph},
	Frame,
};

use super::utils::{centered_rect, rounded_block};

pub fn about(f: &mut Frame) {
	let area =
		centered_rect(f.size(), Constraint::Length(50), Constraint::Length(12));

	f.render_widget(Clear, area);

	let block = rounded_block(Some("ABOUT")).padding(Padding::vertical(3));

	let block_inner = block.inner(area);

	f.render_widget(block, area);

	let p = Paragraph::new(Text::from(vec![
		Line::raw("GitHub"),
		Line::raw("https://github.com/shixinhuang99/sxtetris"),
	]))
	.centered()
	.style(Style::new().white().bold());

	f.render_widget(p, block_inner);
}
