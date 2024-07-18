pub mod bag;
pub mod focus;
pub mod game_over_menu;
pub mod ghost_tetromino;
pub mod main_board;
pub mod next_board;
pub mod pause_menu;
pub mod scores;
pub mod setting_menu;
pub mod start_menu;
pub mod stats;
pub mod tetromino;

use bag::Bag;
use focus::{Focus, Scene};
use game_over_menu::{game_over_menu_idx, GameOverMenu};
use ghost_tetromino::GhostTetromino;
use main_board::{MainBoard, SharedMainBoard};
use next_board::NextBoard;
use pause_menu::{pause_menu_idx, PauseMenu};
use scores::Scores;
use setting_menu::SettingMenu;
use start_menu::{start_menu_idx, StartMenu};
use stats::Stats;
use tetromino::{Tetromino, TetrominoAction};

use crate::{
	common::{Menu, Reset},
	consts::MAIN_BOARD_ROWS,
	global::is_locked,
	handler::{Event, SubHandler},
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
		if self.board.borrow().line_clear.in_progress {
			return;
		}

		match self.focus.current() {
			Scene::StartMenu => self.handle_start_menu(event),
			Scene::Game => self.handle_game_play(event),
			Scene::PauseMenu => self.handle_pause_menu(event),
			Scene::SettingMenu => self.handle_setting_menu(event),
			Scene::GameOverMenu => self.handle_game_over_menu(event),
			Scene::Scores | Scene::Help | Scene::About => {
				if event == Event::Esc {
					self.focus.back();
				}
			}
		}
	}

	fn handle_start_menu(&mut self, event: Event) {
		use start_menu_idx::*;

		match event {
			Event::Up => self.start_menu.up(),
			Event::Down => self.start_menu.down(),
			Event::Enter => {
				match *self.start_menu.cursor_mut() {
					PLAY => self.play(),
					SCORES => self.focus.push(Scene::Scores),
					SETTING => self.focus.push(Scene::SettingMenu),
					HELP => self.focus.push(Scene::Help),
					ABOUT => self.focus.push(Scene::About),
					QUIT => self.running = false,
					_ => (),
				}
			}
			Event::Esc => self.running = false,
			_ => (),
		}
	}

	fn play(&mut self) {
		if self.count_down > 0 {
			self.focus.to(Scene::Game);
			self.update_ghost_tetromino();
			self.handler.pause();
			self.handler.spawn_count_down_task(self.count_down);
		} else {
			self.new_game();
		}
	}

	fn new_game(&mut self) {
		self.focus.to(Scene::Game);
		self.board.borrow_mut().reset();
		self.next_board.reset();
		self.bag.reset();
		self.stats.reset();
		self.pause_menu.reset();
		self.game_over_menu.reset();
		self.alive_tetromino.set_next(self.bag.next());
		self.next_board.set_next(self.bag.next());
		self.update_ghost_tetromino();
		self.handler.spawn_gravity_task();
		self.handler.cancel_pause();
	}

	fn handle_game_play(&mut self, event: Event) {
		use TetrominoAction::*;

		if event == Event::CountDown {
			self.count_down -= 1;
			if self.count_down == 0 {
				self.handler.cancel_pause();
				self.handler.spawn_gravity_task();
				self.check_lock();
			}
			return;
		}

		if self.count_down > 0 {
			return;
		}

		let mut changed = false;

		match event {
			Event::Left => {
				changed = self.alive_tetromino.walk(WalkLeft);
				self.update_ghost_tetromino();
			}
			Event::Right => {
				changed = self.alive_tetromino.walk(WalkRight);
				self.update_ghost_tetromino();
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
				self.lock_tetromino();
			}
			Event::Up => {
				changed = self.alive_tetromino.rotate(RotateRight);
				self.update_ghost_tetromino();
			}
			Event::Z => {
				changed = self.alive_tetromino.rotate(RotateLeft);
				self.update_ghost_tetromino();
			}
			Event::Esc | Event::P | Event::FocusLost => {
				self.handler.pause();
				self.focus.push(Scene::PauseMenu);
			}
			Event::LockEnd => {
				self.lock_tetromino();
			}
			Event::Blink => {
				self.alive_tetromino.blink = !self.alive_tetromino.blink;
			}
			_ => (),
		};

		if changed {
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
			if !virtual_tetromino.walk(TetrominoAction::SoftDrop)
				|| self
					.board
					.borrow()
					.is_collision(&virtual_tetromino.position)
			{
				break;
			}
			max_distance -= 1;
		}

		self.ghost_tetromino
			.position
			.clone_from(&virtual_tetromino.position);
	}

	fn lock_tetromino(&mut self) {
		let cleared_lines = self
			.board
			.borrow_mut()
			.lock_tetromino(&self.alive_tetromino);
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
		let idx = self.scores.push_new_score(self.stats.score);
		self.game_over_menu.set_new_score(self.stats.score, idx);
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

	fn handle_pause_menu(&mut self, event: Event) {
		use pause_menu_idx::*;

		match event {
			Event::Up => self.pause_menu.up(),
			Event::Down => self.pause_menu.down(),
			Event::Enter => {
				match *self.pause_menu.cursor_mut() {
					RESUME => self.handler.cancel_pause(),
					NEW_GAME => {
						self.handler.cancel_lock();
						self.handler.cancel_grvity();
						self.new_game();
					}
					SCORES => self.focus.push(Scene::Scores),
					SETTING => self.focus.push(Scene::SettingMenu),
					HELP => self.focus.push(Scene::Help),
					QUIT => self.running = false,
					_ => (),
				}
			}
			Event::Esc => {
				self.focus.to(Scene::Game);
				self.handler.cancel_pause();
				self.pause_menu.reset();
			}
			_ => (),
		}
	}

	fn handle_setting_menu(&mut self, event: Event) {
		match event {
			Event::Up => self.setting_menu.up(),
			Event::Down => self.setting_menu.down(),
			Event::Enter => self.setting_menu.handle_enter(),
			Event::Esc => self.focus.back(),
			_ => (),
		}
	}

	fn handle_game_over_menu(&mut self, event: Event) {
		use game_over_menu_idx::*;

		match event {
			Event::Up => self.game_over_menu.up(),
			Event::Down => self.game_over_menu.down(),
			Event::Enter => {
				match *self.game_over_menu.cursor_mut() {
					NEW_GAME => self.new_game(),
					SCORES => self.focus.push(Scene::Scores),
					QUIT => self.running = false,
					_ => (),
				}
			}
			Event::Esc => self.focus.back(),
			_ => (),
		}
	}
}
