use ratatui::{
	layout::{Constraint, Layout, Rect},
	Frame,
};

use super::cell::{
	dark_tetromino_cell, empty_cell, ghost_cell, tetromino_cell,
};
use crate::{
	common::Board,
	consts::{
		MAIN_BOARD_BUFFER_ROWS, MAIN_BOARD_COLS, MAIN_BOARD_VISIBLE_ROWS,
	},
	state::State,
};

pub fn main_board(
	f: &mut Frame,
	rect: Rect,
	state: &mut State,
	cell_height: u16,
	cell_width: u16,
) {
	let mut board = state.board.borrow_mut();

	let v_chunks = Layout::vertical(
		[Constraint::Length(cell_height); MAIN_BOARD_VISIBLE_ROWS],
	)
	.areas::<MAIN_BOARD_VISIBLE_ROWS>(rect);

	for (y, v_area) in v_chunks.into_iter().enumerate() {
		let h_chunks = Layout::horizontal(
			[Constraint::Length(cell_width); MAIN_BOARD_COLS],
		)
		.areas::<MAIN_BOARD_COLS>(v_area);

		let y = y + MAIN_BOARD_BUFFER_ROWS;

		for (x, h_area) in h_chunks.into_iter().enumerate() {
			if state.alive_tetromino.position.contains(x, y) {
				let kind = &state.alive_tetromino.kind;
				if state.alive_tetromino.blink {
					dark_tetromino_cell(f, h_area, kind);
				} else {
					tetromino_cell(f, h_area, kind);
				}
			} else if state.ghost_tetromino.position.contains(x, y) {
				ghost_cell(f, h_area, &state.ghost_tetromino.kind);
			} else if let Some(kind) = board.get_kind(x, y) {
				tetromino_cell(f, h_area, kind);
			} else {
				empty_cell(f, h_area);
			}

			if board.particles.check_and_remove_point(x, y) {
				board.particles.spawn(
					h_area.x + h_area.width / 2,
					h_area.y + h_area.height / 2,
				);
			}
		}
	}
}
