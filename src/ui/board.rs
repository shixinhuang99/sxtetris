use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::Style,
	widgets::{Block, BorderType},
	Frame,
};

use crate::{consts::BOARD_VISIBLE_ROWS, state::State};

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

	for (y, v_area) in v_chunks.iter().enumerate() {
		let h_chunks = Layout::horizontal(vec![
			Constraint::Length(cell_width);
			state.board.cols
		])
		.split(*v_area);

		let y = y + BOARD_VISIBLE_ROWS;

		for (x, h_area) in h_chunks.iter().enumerate() {
			let dirs = state.confetti_state.get_point_dirs(x, y);
			state.confetti_state.spawn_particles(
				h_area.x,
				h_area.y,
				h_area.width,
				h_area.height,
				dirs,
			);

			let tm_type = state.board.get_cell(x, y);

			let tm_color = tm_type.color();
			let piece_style = Style::new().fg(tm_color);

			if tm_type.is_none_or_ghost() {
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
