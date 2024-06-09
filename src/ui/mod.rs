mod list;
mod matrix;
mod pause_menu;
mod scores;
mod sidebar;
mod start_menu;
mod utils;

use matrix::matrix;
use pause_menu::pause_menu;
use ratatui::{
	layout::{Constraint, Flex, Layout},
	Frame,
};
#[cfg(feature = "_dev")]
use ratatui::{text::Line, widgets::Paragraph};
use scores::scores;
use sidebar::sidebar;
use start_menu::start_menu;

use crate::{
	consts::{
		MATRIX_X_LEN, MATRIX_X_LEN_U16, MATRIX_Y_VISIBLE_LEN,
		MATRIX_Y_VISIBLE_LEN_U16, MIN_CELL_HEIGHT, MIN_CELL_WIDTH,
	},
	state::{CurrentlyScreen, State},
};

pub fn ui(f: &mut Frame, state: &State) {
	let screen = f.size();

	if matches!(state.currently_screen, CurrentlyScreen::StartMenu) {
		start_menu(f, screen, &state.start_menu);

		if state.show_scores {
			scores(f, state);
		}

		return;
	}

	let (cell_height, cell_width) = calc_cell_size(screen.height);

	#[cfg(feature = "_dev")]
	f.render_widget(
		Paragraph::new(vec![
			Line::raw(format!("cell_height: {}", cell_height)),
			Line::raw(format!("cell_width: {}", cell_width)),
		]),
		screen,
	);

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

	matrix(
		f,
		left_area,
		[Constraint::Length(cell_height); MATRIX_Y_VISIBLE_LEN],
		[Constraint::Length(cell_width); MATRIX_X_LEN],
		false,
		|x, y| state.tm_board_mapping(x, y + MATRIX_Y_VISIBLE_LEN),
	);

	sidebar(f, right_area, state, cell_height, cell_width);

	if state.paused {
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
