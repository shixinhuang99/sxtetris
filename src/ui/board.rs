use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Style, Stylize},
	widgets::{Block, BorderType, Borders},
	Frame,
};

use crate::state::{State, TetrominoKind};

pub fn board(
	f: &mut Frame,
	rect: Rect,
	state: &mut State,
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

	let v_chunks =
		Layout::vertical(vec![Constraint::Length(cell_height); rows])
			.split(rect);

	for (y, v_area) in v_chunks.iter().enumerate() {
		let h_chunks = Layout::horizontal(vec![
			Constraint::Length(cell_width);
			board.cols
		])
		.split(*v_area);

		let y_with_offest = if is_main_board {
			y + rows
		} else {
			y
		};

		for (x, h_area) in h_chunks.iter().enumerate() {
			let tm_kind = board.get_cell(x, y_with_offest);

			let piece_style = create_style(tm_kind);

			let mut piece = if tm_kind.is_none_or_ghost() {
				Block::bordered()
					.border_type(BorderType::Rounded)
					.border_style(piece_style)
			} else {
				let mut outer =
					Block::bordered().border_type(BorderType::QuadrantInside);

				let inner_area = outer.inner(*h_area);
				let mut inside = Block::new().style(create_style_bg(tm_kind));

				if state.active_tm.points.contains(x, y_with_offest)
					&& state.blinking
				{
					outer = outer.dark_gray();
					inside = inside.on_dark_gray();
				} else {
					outer = outer.border_style(piece_style);
				}

				f.render_widget(inside, inner_area);

				outer
			};

			if !is_main_board && *tm_kind == TetrominoKind::None {
				piece = piece.borders(Borders::NONE);
			}

			f.render_widget(piece, *h_area);

			let dirs = state.confetti_state.get_point_dirs(x, y_with_offest);
			state.confetti_state.spawn_particles(
				h_area.x,
				h_area.y,
				h_area.width,
				h_area.height,
				dirs,
			);
		}
	}
}

fn create_style(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().cyan(),
		TetrominoKind::O => Style::new().light_yellow(),
		TetrominoKind::T => Style::new().magenta(),
		TetrominoKind::L => Style::new().yellow(),
		TetrominoKind::J => Style::new().blue(),
		TetrominoKind::S => Style::new().green(),
		TetrominoKind::Z => Style::new().red(),
		TetrominoKind::None => Style::new().dark_gray(),
		TetrominoKind::Ghost => Style::new().gray(),
	}
}

fn create_style_bg(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().on_cyan(),
		TetrominoKind::O => Style::new().on_light_yellow(),
		TetrominoKind::T => Style::new().on_magenta(),
		TetrominoKind::L => Style::new().on_yellow(),
		TetrominoKind::J => Style::new().on_blue(),
		TetrominoKind::S => Style::new().on_green(),
		TetrominoKind::Z => Style::new().on_red(),
		_ => unreachable!(),
	}
}
