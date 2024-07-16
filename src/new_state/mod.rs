mod bag;
mod game_over_menu;
mod ghost_tetromino;
mod main_board;
mod next_board;
mod pause_menu;
mod setting_menu;
mod start_menu;
mod tetromino;

use std::{cell::RefCell, rc::Rc};

use bag::Bag;
use game_over_menu::GameOverMenu;
use ghost_tetromino::GhostTetromino;
use main_board::MainBoard;
use next_board::NextBoard;
use pause_menu::PauseMenu;
use start_menu::StartMenu;
use tetromino::Tetromino;

use crate::handler::SubHandler;

#[derive(PartialEq, Eq)]
pub enum Focus {
	StartMenu,
	Game,
	PauseMenu,
	GameOverMenu,
	SettingMenu,
	Scores,
	Help,
	About,
}

pub struct State {
	handler: SubHandler,
	pub running: bool,
	pub focus: Focus,
	pub start_menu: StartMenu,
	pub pause_menu: PauseMenu,
	pub game_over_menu: GameOverMenu,
	pub bag: Bag,
	pub board: Rc<RefCell<MainBoard>>,
	pub next_board: NextBoard,
	pub alive_tetromino: Tetromino,
	pub ghost_tetromino: GhostTetromino,
	pub count_down: u8,
}

impl State {
	pub fn new(handler: SubHandler) -> Self {
		let board = Rc::new(RefCell::new(MainBoard::new()));
		let alive_tetromino = Tetromino::new(board.clone());

		Self {
			handler,
			running: true,
			focus: Focus::StartMenu,
			start_menu: StartMenu::new(),
			pause_menu: PauseMenu::new(),
			game_over_menu: GameOverMenu::new(),
			bag: Bag::new(),
			board,
			next_board: NextBoard::new(),
			alive_tetromino,
			ghost_tetromino: GhostTetromino::default(),
			count_down: 0,
		}
	}
}
