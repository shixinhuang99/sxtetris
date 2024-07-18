use ratatui::{
	layout::{Constraint, Layout, Rect},
	Frame,
};

use super::cell::tetromino_cell;
use crate::{
	common::Board,
	consts::{NEXT_BOARD_COLS, NEXT_BOARD_ROWS},
	new_state::next_board::NextBoard,
};

pub fn next_board(
	f: &mut Frame,
	rect: Rect,
	next_board: &NextBoard,
	cell_height: u16,
	cell_width: u16,
) {
	let v_chunks =
		Layout::vertical([Constraint::Length(cell_height); NEXT_BOARD_ROWS])
			.areas::<NEXT_BOARD_ROWS>(rect);

	for (y, v_area) in v_chunks.into_iter().enumerate() {
		let h_chunks = Layout::horizontal(
			[Constraint::Length(cell_width); NEXT_BOARD_COLS],
		)
		.areas::<NEXT_BOARD_COLS>(v_area);

		for (x, h_area) in h_chunks.into_iter().enumerate() {
			if let Some(kind) = next_board.get_kind(x, y) {
				tetromino_cell(f, h_area, kind);
			};
		}
	}
}
