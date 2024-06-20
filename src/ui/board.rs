use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders},
	Frame,
};

use super::utils::rounded_block;
use crate::state::{State, TetrominoKind};

pub fn board(
	f: &mut Frame,
	rect: Rect,
	state: &State,
	cell_height: u16,
	cell_width: u16,
	is_main_board: bool,
) {
	let board = if is_main_board {
		&state.board
	} else {
		&state.preview_board
	};

	let rows = if is_main_board {
		board.rows / 2
	} else {
		board.rows
	};

	let vertical_chunks =
		Layout::vertical(vec![Constraint::Length(cell_height); rows])
			.split(rect);

	for (y, vertical_area) in vertical_chunks.iter().enumerate() {
		let horinzontal_chunks =
			Layout::horizontal(vec![
				Constraint::Length(cell_width);
				board.cols
			])
			.split(*vertical_area);

		for (x, horizontal_area) in horinzontal_chunks.iter().enumerate() {
			let y_with_offest = if is_main_board {
				y + rows
			} else {
				y
			};

			let tm_kind = board.get_cell(x, y_with_offest);

			let piece_style = create_style(tm_kind);

			let mut piece = if tm_kind.is_none_or_ghost() {
				rounded_block(None).border_style(piece_style)
			} else {
				let mut outer = Block::new()
					.borders(Borders::ALL)
					.border_type(BorderType::QuadrantInside);

				let inner_area = outer.inner(*horizontal_area);
				let mut inside = Block::new().style(create_style_bg(tm_kind));

				if state.active_tm.points.contains(x, y_with_offest)
					&& state.blinking
				{
					outer =
						outer.border_style(Style::new().fg(Color::DarkGray));
					inside = inside.style(Style::new().bg(Color::DarkGray));
				} else {
					outer = outer.border_style(piece_style);
				}

				f.render_widget(inside, inner_area);

				outer
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
