mod about;
mod board;
mod confetti;
mod count_down;
mod game_over_menu;
mod help;
mod list;
mod pause_menu;
mod preview_board;
mod scores;
mod sidebar;
mod start_menu;
mod utils;

use about::about;
use board::board;
use confetti::Confetti;
use count_down::count_down;
use game_over_menu::game_over_menu;
use help::help;
use pause_menu::pause_menu;
use ratatui::{
	layout::{Constraint, Flex, Layout},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders},
	Frame,
};
use scores::scores;
use sidebar::sidebar;
use start_menu::start_menu;

use crate::{
	consts::{BOARD_COLS, BOARD_VISIBLE_ROWS, MIN_CELL_HEIGHT, MIN_CELL_WIDTH},
	handler::is_paused,
	state::{Screen, State},
};

const BOARD_VISIBLE_ROWS_U16: u16 = BOARD_VISIBLE_ROWS as u16;

const BOARD_COLS_U16: u16 = BOARD_COLS as u16;

pub fn ui(f: &mut Frame, state: &mut State) {
	let screen = f.size();

	let bg_block = Block::new()
		.borders(Borders::NONE)
		.border_type(BorderType::Plain)
		.style(Style::new().bg(Color::Black));

	f.render_widget(bg_block, screen);

	if state.screen == Screen::StartMenu {
		start_menu(f, screen, &state.start_menu);

		if state.show_scores {
			scores(f, state);
		} else if state.show_help {
			help(f);
		} else if state.show_about {
			about(f);
		}

		return;
	}

	let (cell_height, cell_width) = calc_cell_size(screen.height);

	let vertical_area = Layout::vertical([Constraint::Length(
		cell_height * BOARD_VISIBLE_ROWS_U16,
	)])
	.flex(Flex::Center)
	.areas::<1>(screen)[0];

	let horizontal_area = Layout::horizontal([
		Constraint::Percentage(50),
		Constraint::Percentage(50),
	])
	.areas::<2>(vertical_area);

	let left_area =
		Layout::horizontal([Constraint::Length(cell_width * BOARD_COLS_U16)])
			.flex(Flex::End)
			.areas::<1>(horizontal_area[0])[0];

	let right_area =
		Layout::horizontal([Constraint::Length(cell_width * BOARD_COLS_U16)])
			.flex(Flex::Start)
			.areas::<1>(horizontal_area[1])[0];

	board(f, left_area, state, cell_height, cell_width);

	let confetti = Confetti;
	f.render_stateful_widget(confetti, f.size(), &mut state.board.confetti);

	sidebar(f, right_area, state, cell_height, cell_width);

	if state.is_game_over {
		game_over_menu(f, state);
	} else if state.count_down > 0 {
		count_down(f, state);
	} else if is_paused() {
		pause_menu(f, &state.pause_menu);
	}

	if state.show_scores {
		scores(f, state);
	} else if state.show_help {
		help(f);
	}
}

fn calc_cell_size(screen_height: u16) -> (u16, u16) {
	let height = MIN_CELL_HEIGHT.max(screen_height / BOARD_VISIBLE_ROWS_U16);
	let width = MIN_CELL_WIDTH.max((height as f32 * 1.8) as u16);

	(height, width)
}
