mod about;
mod board;
mod cell;
mod count_down;
mod game_over_menu;
mod help;
mod menu;
mod next_board;
mod pause_menu;
mod scores;
mod setting_menu;
mod sidebar;
mod start_menu;
mod utils;

use about::about;
use board::main_board;
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
use setting_menu::setting_menu;
use sidebar::sidebar;
use start_menu::start_menu;

use crate::{
	consts::{
		MAIN_BOARD_COLS, MAIN_BOARD_VISIBLE_ROWS, MIN_CELL_HEIGHT,
		MIN_CELL_WIDTH,
	},
	state::{focus::Scene, State},
};

const ROWS: u16 = MAIN_BOARD_VISIBLE_ROWS as u16;
const COLS: u16 = MAIN_BOARD_COLS as u16;

pub fn ui(f: &mut Frame, state: &mut State) {
	let screen = f.size();

	let bg_block = Block::new()
		.borders(Borders::NONE)
		.border_type(BorderType::Plain)
		.style(Style::new().bg(Color::Black));

	f.render_widget(bg_block, screen);

	if state.focus.contains(Scene::StartMenu) {
		start_menu(f, screen, &state.start_menu);
	}

	if state.focus.contains(Scene::Game) {
		let (cell_height, cell_width) = calc_cell_size(screen.height);

		let vertical_area =
			Layout::vertical([Constraint::Length(cell_height * ROWS)])
				.flex(Flex::Center)
				.areas::<1>(screen)[0];

		let horizontal_area = Layout::horizontal([
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		])
		.areas::<2>(vertical_area);

		let left_area =
			Layout::horizontal([Constraint::Length(cell_width * COLS)])
				.flex(Flex::End)
				.areas::<1>(horizontal_area[0])[0];

		let right_area =
			Layout::horizontal([Constraint::Length(cell_width * COLS)])
				.flex(Flex::Start)
				.areas::<1>(horizontal_area[1])[0];

		main_board(f, left_area, state, cell_height, cell_width);

		sidebar(f, right_area, state, cell_height, cell_width);

		if state.count_down != 0 {
			count_down(f, state.count_down);
		}
	}

	if state.focus.contains(Scene::GameOverMenu) {
		game_over_menu(f, &state.game_over_menu);
	}

	if state.focus.contains(Scene::PauseMenu) {
		pause_menu(f, &state.pause_menu);
	}

	if state.focus.contains(Scene::Scores) {
		scores(f, &state.scores);
	}

	if state.focus.contains(Scene::SettingMenu) {
		setting_menu(f, &state.setting_menu);
	}

	if state.focus.contains(Scene::Help) {
		help(f);
	}

	if state.focus.contains(Scene::About) {
		about(f);
	}
}

fn calc_cell_size(screen_height: u16) -> (u16, u16) {
	let height = MIN_CELL_HEIGHT.max(screen_height / ROWS);
	let width = MIN_CELL_WIDTH.max((height as f32 * 1.8) as u16);

	(height, width)
}
