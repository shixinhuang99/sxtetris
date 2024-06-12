use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	symbols,
	text::Text,
	widgets::{Borders, Paragraph},
	Frame,
};

use super::utils::rounded_block;
use crate::state::{BoardState, TetrominoKind};

pub fn board(
	f: &mut Frame,
	rect: Rect,
	board_state: &BoardState,
	cell_height: u16,
	cell_width: u16,
	is_main_board: bool,
) {
	let rows = if is_main_board {
		board_state.rows / 2
	} else {
		board_state.rows
	};

	let vertical_chunks =
		Layout::vertical(vec![Constraint::Length(cell_height); rows])
			.split(rect);

	for (y, vertical_area) in vertical_chunks.iter().enumerate() {
		let horinzontal_chunks =
			Layout::horizontal(vec![
				Constraint::Length(cell_width);
				board_state.cols
			])
			.split(*vertical_area);

		for (x, horizontal_area) in horinzontal_chunks.iter().enumerate() {
			let y_with_offest = if is_main_board {
				y + rows
			} else {
				y
			};

			let tm_kind = board_state.get_cell(x, y_with_offest);

			let style = create_style(tm_kind);

			let mut b = rounded_block(None).border_style(style);

			if !is_main_board && *tm_kind == TetrominoKind::None {
				b = b.borders(Borders::NONE);
			}

			#[cfg(feature = "_dev")]
			{
				b = b
					.title_top(x.to_string())
					.title_bottom(y_with_offest.to_string());
			}

			let p = Paragraph::new(create_text(tm_kind)).block(b).style(style);

			f.render_widget(p, *horizontal_area);
		}
	}
}

fn create_style(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().fg(Color::Indexed(14)),
		TetrominoKind::O => Style::new().fg(Color::Indexed(208)),
		TetrominoKind::T => Style::new().fg(Color::Indexed(13)),
		TetrominoKind::L => Style::new().fg(Color::Indexed(202)),
		TetrominoKind::J => Style::new().fg(Color::Indexed(12)),
		TetrominoKind::S => Style::new().fg(Color::Indexed(10)),
		TetrominoKind::Z => Style::new().fg(Color::Indexed(9)),
		TetrominoKind::None => Style::new().fg(Color::Black),
		TetrominoKind::Ghost => Style::new().fg(Color::White),
	}
}

fn create_text(tm_kind: &TetrominoKind) -> Text<'static> {
	let content =
		if matches!(tm_kind, TetrominoKind::None | TetrominoKind::Ghost) {
			' '
		} else {
			symbols::half_block::UPPER
		};

	#[cfg(feature = "_dev")]
	let content = char::from(*tm_kind);

	Text::raw(content.to_string())
}
