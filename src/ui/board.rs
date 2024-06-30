use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::Style,
	widgets::{Block, BorderType},
	Frame,
};

use crate::{
	consts::BOARD_VISIBLE_ROWS,
	state::{State, TetrominoType},
};

pub fn board(
	f: &mut Frame,
	rect: Rect,
	state: &mut State,
	cell_height: u16,
	cell_width: u16,
) {
	let v_chunks = Layout::vertical(vec![
		Constraint::Length(cell_height);
		BOARD_VISIBLE_ROWS
	])
	.split(rect);

	let ghost_style = get_ghost_style(&state.active_tm.tm_type);

	for (y, v_area) in v_chunks.iter().enumerate() {
		let h_chunks = Layout::horizontal(vec![
			Constraint::Length(cell_width);
			state.board.cols
		])
		.split(*v_area);

		let y = y + BOARD_VISIBLE_ROWS;

		for (x, h_area) in h_chunks.iter().enumerate() {
			if state.board.confetti.is_target_point(x, y) {
				state.board.confetti.spawn_particles(
					h_area.x,
					h_area.y,
					h_area.width,
					h_area.height,
				);
			}

			let tm_type = state.board.get_cell(x, y);

			let tm_color = tm_type.color();
			let piece_style = Style::new().fg(tm_color);

			if *tm_type == TetrominoType::Ghost {
				f.render_widget(
					Block::bordered()
						.border_type(BorderType::Rounded)
						.border_style(ghost_style),
					*h_area,
				);
				continue;
			}

			if *tm_type == TetrominoType::None {
				f.render_widget(
					Block::bordered()
						.border_type(BorderType::Rounded)
						.border_style(piece_style),
					*h_area,
				);
				continue;
			}

			let mut outer_block =
				Block::bordered().border_type(BorderType::QuadrantInside);
			let inner_area = outer_block.inner(*h_area);
			let mut inside_block =
				Block::new().style(Style::new().bg(tm_color));

			if state.active_tm.points.contains(x, y) && state.blinking {
				let tm_dark_color = tm_type.dark_color();
				outer_block = outer_block.style(Style::new().fg(tm_dark_color));
				inside_block =
					inside_block.style(Style::new().bg(tm_dark_color));
			} else {
				outer_block = outer_block.border_style(piece_style);
			}

			f.render_widget(inside_block, inner_area);
			f.render_widget(outer_block, *h_area);
		}
	}
}

// The borders get bigger when using RGB colors, so here we use ANSI colors
fn get_ghost_style(tm_type: &TetrominoType) -> Style {
	use ratatui::style::Stylize;

	match tm_type {
		TetrominoType::I => Style::new().cyan(),
		TetrominoType::O => Style::new().light_yellow(),
		TetrominoType::T => Style::new().magenta(),
		TetrominoType::L => Style::new().yellow(),
		TetrominoType::J => Style::new().light_blue(),
		TetrominoType::S => Style::new().green(),
		TetrominoType::Z => Style::new().light_red(),
		_ => unreachable!(),
	}
}
