use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders},
	Frame,
};

use super::utils::rounded_block;
use crate::state::{BoardState, Tetromino, TetrominoKind};

pub fn board(
	f: &mut Frame,
	rect: Rect,
	board_state: &BoardState,
	cell_height: u16,
	cell_width: u16,
	is_main_board: bool,
	active_tm: &Tetromino,
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

			let piece_style = create_style(tm_kind);

			let mut piece = if !matches!(
				tm_kind,
				TetrominoKind::None | TetrominoKind::Ghost
			) {
				let mut outer = Block::new()
					.borders(Borders::ALL)
					.border_type(BorderType::QuadrantInside);

				let inner_area = outer.inner(*horizontal_area);
				let mut insider = Block::new().style(create_style_bg(tm_kind));

				if active_tm.points.contains(x, y_with_offest)
					&& active_tm.is_blink
				{
					outer =
						outer.border_style(Style::new().fg(Color::DarkGray));
					insider = insider.style(Style::new().bg(Color::DarkGray));
				} else {
					outer = outer.border_style(piece_style);
				}

				f.render_widget(insider, inner_area);

				outer
			} else {
				rounded_block(None).border_style(piece_style)
			};

			if !is_main_board && *tm_kind == TetrominoKind::None {
				piece = piece.borders(Borders::NONE);
			}

			f.render_widget(piece, *horizontal_area);
		}
	}
}

fn create_style(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().fg(Color::Cyan),
		TetrominoKind::O => Style::new().fg(Color::LightYellow),
		TetrominoKind::T => Style::new().fg(Color::Magenta),
		TetrominoKind::L => Style::new().fg(Color::Yellow),
		TetrominoKind::J => Style::new().fg(Color::Blue),
		TetrominoKind::S => Style::new().fg(Color::Green),
		TetrominoKind::Z => Style::new().fg(Color::Red),
		TetrominoKind::None => Style::new().fg(Color::DarkGray),
		TetrominoKind::Ghost => Style::new().fg(Color::Gray),
	}
}

fn create_style_bg(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().bg(Color::Cyan),
		TetrominoKind::O => Style::new().bg(Color::LightYellow),
		TetrominoKind::T => Style::new().bg(Color::Magenta),
		TetrominoKind::L => Style::new().bg(Color::Yellow),
		TetrominoKind::J => Style::new().bg(Color::Blue),
		TetrominoKind::S => Style::new().bg(Color::Green),
		TetrominoKind::Z => Style::new().bg(Color::Red),
		_ => unreachable!(),
	}
}
