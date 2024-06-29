use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::Style,
	widgets::{Block, BorderType},
	Frame,
};

use crate::state::State;

pub fn preview_board(
	f: &mut Frame,
	rect: Rect,
	state: &mut State,
	cell_height: u16,
	cell_width: u16,
) {
	let v_chunks = Layout::vertical(vec![
		Constraint::Length(cell_height);
		state.preview_board.rows
	])
	.split(rect);

	for (y, v_area) in v_chunks.iter().enumerate() {
		let h_chunks = Layout::horizontal(vec![
			Constraint::Length(cell_width);
			state.preview_board.cols
		])
		.split(*v_area);

		for (x, h_area) in h_chunks.iter().enumerate() {
			let tm_type = state.preview_board.get_cell(x, y);

			if tm_type.is_none_or_ghost() {
				continue;
			}

			let tm_color = tm_type.color();

			let outer_block = Block::bordered()
				.border_type(BorderType::QuadrantInside)
				.border_style(Style::new().fg(tm_color));

			let inner_area = outer_block.inner(*h_area);
			let inside_block = Block::new().style(Style::new().bg(tm_color));

			f.render_widget(inside_block, inner_area);
			f.render_widget(outer_block, *h_area);
		}
	}
}
