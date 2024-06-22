use ratatui::{
	layout::Constraint,
	style::{Style, Stylize},
	text::{Line, Text},
	widgets::{block::Padding, Clear, Paragraph},
	Frame,
};

use super::utils::{centered_rect, rounded_block};

pub fn help(f: &mut Frame) {
	let area =
		centered_rect(f.size(), Constraint::Length(36), Constraint::Length(13));

	f.render_widget(Clear, area);

	let block = rounded_block(Some("HELP")).padding(Padding::vertical(2));

	let block_inner = block.inner(area);

	f.render_widget(block, area);

	let lines: Vec<Line> = HELP_TEXT
		.iter()
		.map(|t| Line::from(format!("{:<17}{:<15}", t[0], t[1])))
		.collect();

	let p = Paragraph::new(Text::from(lines))
		.centered()
		.style(Style::new().white().bold());

	f.render_widget(p, block_inner);
}

const HELP_TEXT: [[&str; 2]; 7] = [
	["MOVE RIGHT", "RIGHT ARROW / L"],
	["MOVE LEFT", "LEFT ARROW / J"],
	["ROTATA RIGHT", "UP ARROW / I"],
	["ROTATE LEFT", "Z"],
	["SOFT DROP", "DOWN ARROW / K"],
	["HARD DROP", "SPACE"],
	["PAUSE", "ESC / P"],
];
