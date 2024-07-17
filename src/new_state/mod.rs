mod bag;
mod focus;
mod game_over_menu;
mod ghost_tetromino;
mod main_board;
mod next_board;
mod pause_menu;
mod scores;
mod setting_menu;
mod start_menu;
mod stats;
mod tetromino;

use bag::Bag;
use focus::{Focus, Scene};
use game_over_menu::GameOverMenu;
use ghost_tetromino::GhostTetromino;
use main_board::{LineClearStatus, MainBoard, SharedMainBoard};
use next_board::NextBoard;
use pause_menu::PauseMenu;
use scores::Scores;
use setting_menu::SettingMenu;
use start_menu::StartMenu;
use stats::Stats;
use tetromino::{Tetromino, TetrominoAction};

use crate::{
	common::Menu,
	consts::MAIN_BOARD_ROWS,
	handler::{is_locked, is_paused, Event, SubHandler},
};

pub struct State {
	handler: SubHandler,
	pub running: bool,
	pub focus: Focus,
	pub start_menu: StartMenu,
	pub pause_menu: PauseMenu,
	pub game_over_menu: GameOverMenu,
	pub setting_menu: SettingMenu,
	pub bag: Bag,
	pub board: SharedMainBoard,
	pub next_board: NextBoard,
	pub alive_tetromino: Tetromino,
	pub ghost_tetromino: GhostTetromino,
	pub count_down: u8,
	pub stats: Stats,
	pub scores: Scores,
}

impl State {
	pub fn new(handler: SubHandler) -> Self {
		let board = MainBoard::new_shared();
		let alive_tetromino = Tetromino::new(board.clone());

		Self {
			handler,
			running: true,
			focus: Focus::new(),
			start_menu: StartMenu::new(),
			pause_menu: PauseMenu::new(),
			game_over_menu: GameOverMenu::new(),
			setting_menu: SettingMenu::new(),
			bag: Bag::new(),
			board,
			next_board: NextBoard::new(),
			alive_tetromino,
			ghost_tetromino: GhostTetromino::default(),
			count_down: 0,
			stats: Stats::new(),
			scores: Scores::new(),
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		if self.board.borrow().line_clear.status == LineClearStatus::Pending {
			return;
		}

		match self.focus.current() {
			Scene::Game => self.handle_game(event),
			_ => unimplemented!(),
		}
	}

	fn handle_game(&mut self, event: Event) {
		use TetrominoAction::*;

		let mut changed = false;

		match &event {
			Event::Left => {
				changed = self.alive_tetromino.walk(WalkLeft);
			}
			Event::Right => {
				changed = self.alive_tetromino.walk(WalkRight);
			}
			Event::Down | Event::Gravity => {
				changed = self.alive_tetromino.walk(SoftDrop);
				self.stats.score += 1;
			}
			Event::Space => {
				let y1 = self.ghost_tetromino.position.bottom_point().y;
				let y2 = self.alive_tetromino.position.bottom_point().y;
				self.alive_tetromino
					.position
					.clone_from(&self.ghost_tetromino.position);
				self.stats.score += (y1 - y2) as u32 * 2;
				self.handler.cancel_lock();
				self.before_next_alive_tetromino();
			}
			Event::Up => {
				changed = self.alive_tetromino.rotate(RotateRight);
			}
			Event::Z => {
				changed = self.alive_tetromino.rotate(RotateLeft);
			}
			Event::Esc | Event::P | Event::FocusLost => {
				self.handler.pause();
			}
			Event::LockEnd => {
				self.before_next_alive_tetromino();
			}
			Event::Blink => {
				self.alive_tetromino.blink = !self.alive_tetromino.blink;
			}
			Event::CountDown(v) => {
				self.count_down = *v;
				if self.count_down == 0 {
					self.handler.cancel_pause();
					self.handler.spawn_gravity_task();
					self.check_lock();
				}
			}
			_ => (),
		};

		if changed {
			if matches!(
				event,
				Event::Up | Event::Z | Event::Left | Event::Right
			) {
				self.update_ghost_tetromino();
			}
			self.check_lock();
		}
	}

	fn check_lock(&mut self) {
		let fit_together =
			self.alive_tetromino.position == self.ghost_tetromino.position;
		if is_locked() {
			if !fit_together {
				self.alive_tetromino.blink = false;
				self.handler.cancel_lock();
			} else {
				self.handler.refresh_lock();
			}
		} else if fit_together {
			self.handler.spawn_lock_task();
		}
	}

	fn update_ghost_tetromino(&mut self) {
		let bottom_point = self.alive_tetromino.position.bottom_point();
		let mut max_distance = MAIN_BOARD_ROWS as i8 - bottom_point.y - 1;
		let mut virtual_tetromino = self.alive_tetromino.clone();

		while max_distance > 0 {
			if !virtual_tetromino.walk(TetrominoAction::SoftDrop) {
				break;
			}
			max_distance -= 1;
		}

		self.ghost_tetromino
			.position
			.clone_from(&virtual_tetromino.position);
	}

	fn before_next_alive_tetromino(&mut self) {
		let cleared_lines = self.board.borrow_mut().check_line_clear();
		let previous_level = self.stats.level;

		self.stats.update(cleared_lines);

		if self.stats.level > previous_level {
			self.handler.change_level(self.stats.level);
		}

		if cleared_lines != 0 {
			return;
		}

		if self.alive_tetromino.position.is_outside_the_visible() {
			self.game_over();
			return;
		}

		self.next_alive_tetromino();
	}

	fn game_over(&mut self) {
		self.focus.push(Scene::GameOverMenu);
		self.handler.pause();
		self.handler.cancel_grvity();
		self.handler.cancel_lock();
		if let Some(idx) = self.scores.push_new_score(self.stats.score) {
			self.game_over_menu
				.set_new_score(Some((self.stats.score, idx)));
		}
	}

	fn next_alive_tetromino(&mut self) {
		self.alive_tetromino.set_next(self.next_board.current);
		self.next_board.set_next(self.bag.next());

		if self
			.board
			.borrow()
			.is_collision(&self.alive_tetromino.position)
		{
			self.game_over();
			return;
		}

		self.handler.reset_gravity();
	}
}
