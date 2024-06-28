use ratatui::{
	style::{Style, Stylize},
	text::{Line, Text},
	widgets::{block::Padding, Paragraph},
	Frame,
};

use super::utils::Popup;

pub fn help(f: &mut Frame) {
	let popup = Popup::new(36, 13)
		.title("HELP")
		.padding(Padding::vertical(2))
		.render(f);

	let lines: Vec<Line> = HELP_TEXT
		.iter()
		.map(|t| Line::from(format!("{:<17}{:<15}", t[0], t[1])))
		.collect();

	let p = Paragraph::new(Text::from(lines))
		.centered()
		.style(Style::new().white().bold());

	f.render_widget(p, popup);
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
