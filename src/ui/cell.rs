use ratatui::{
	layout::Rect,
	style::{Color, Style},
	widgets::{Block, BorderType},
	Frame,
};

use crate::common::TetrominoKind;

fn cell(f: &mut Frame, rect: Rect, color: Color) {
	let outer = Block::bordered()
		.border_type(BorderType::QuadrantInside)
		.border_style(Style::new().fg(color));
	let inner_area = outer.inner(rect);
	let inside = Block::new().style(Style::new().bg(color));

	f.render_widget(inside, inner_area);
	f.render_widget(outer, rect);
}

pub fn tetromino_cell(f: &mut Frame, rect: Rect, kind: &TetrominoKind) {
	cell(f, rect, kind.color());
}

pub fn dark_tetromino_cell(f: &mut Frame, rect: Rect, kind: &TetrominoKind) {
	cell(f, rect, kind.dark_color());
}

pub fn ghost_cell(f: &mut Frame, rect: Rect, kind: &TetrominoKind) {
	let block = Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(ghost_style(kind));

	f.render_widget(block, rect);
}

pub fn empty_cell(f: &mut Frame, rect: Rect) {
	let block = Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(Color::DarkGray);

	f.render_widget(block, rect);
}

// The borders get bigger when using RGB colors, so here we use ANSI colors
fn ghost_style(kind: &TetrominoKind) -> Style {
	use ratatui::style::Stylize;

	match kind {
		TetrominoKind::I => Style::new().cyan(),
		TetrominoKind::O => Style::new().light_yellow(),
		TetrominoKind::T => Style::new().magenta(),
		TetrominoKind::L => Style::new().yellow(),
		TetrominoKind::J => Style::new().light_blue(),
		TetrominoKind::S => Style::new().green(),
		TetrominoKind::Z => Style::new().light_red(),
	}
}
