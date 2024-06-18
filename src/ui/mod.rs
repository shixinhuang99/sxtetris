mod board;
mod count_down;
mod game_over_menu;
mod list;
mod pause_menu;
mod scores;
mod sidebar;
mod start_menu;
mod utils;

use board::board;
use count_down::count_down;
use game_over_menu::game_over_menu;
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
	consts::{
		BOARD_VISIBLE_Y_LEN, BOARD_X_LEN, MIN_CELL_HEIGHT, MIN_CELL_WIDTH,
	},
	state::{Screen, State},
};

const MATRIX_Y_VISIBLE_LEN_U16: u16 = BOARD_VISIBLE_Y_LEN as u16;

const MATRIX_X_LEN_U16: u16 = BOARD_X_LEN as u16;

pub fn ui(f: &mut Frame, state: &State) {
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
		}

		return;
	}

	let (cell_height, cell_width) = calc_cell_size(screen.height);

	#[cfg(feature = "_dev")]
	{
		log::trace!("screen size: {} x {}", screen.width, screen.height);
		log::trace!("cell size: {} x {}", cell_width, cell_height);
	}

	let vertical_area = Layout::vertical([Constraint::Length(
		cell_height * MATRIX_Y_VISIBLE_LEN_U16,
	)])
	.flex(Flex::Center)
	.areas::<1>(screen)[0];

	let horizontal_area = Layout::horizontal([
		Constraint::Percentage(50),
		Constraint::Percentage(50),
	])
	.areas::<2>(vertical_area);

	let left_area =
		Layout::horizontal([Constraint::Length(cell_width * MATRIX_X_LEN_U16)])
			.flex(Flex::End)
			.areas::<1>(horizontal_area[0])[0];

	let right_area =
		Layout::horizontal([Constraint::Length(cell_width * MATRIX_X_LEN_U16)])
			.flex(Flex::Start)
			.areas::<1>(horizontal_area[1])[0];

	board(
		f,
		left_area,
		&state.board,
		cell_height,
		cell_width,
		true,
		&state.active_tm,
	);

	sidebar(f, right_area, state, cell_height, cell_width);

	if state.is_game_over {
		game_over_menu(f, state);
	} else if state.last_game_count_down > 0 {
		count_down(f, state);
	} else if state.paused {
		pause_menu(f, &state.pause_menu);
	}

	if state.show_scores {
		scores(f, state);
	}
}

fn calc_cell_size(screen_height: u16) -> (u16, u16) {
	let cell_height =
		MIN_CELL_HEIGHT.max(screen_height / MATRIX_Y_VISIBLE_LEN_U16);
	let cell_width = MIN_CELL_WIDTH.max((cell_height as f32 * 1.8) as u16);

	(cell_height, cell_width)
}
