pub mod bag;
pub mod focus;
pub mod game_over_menu;
pub mod ghost_tetromino;
pub mod main_board;
pub mod next_board;
pub mod particles;
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
	global::{global_audio, is_locked, set_played, Sound},
	handler::{Event, SubHandler},
};

pub struct State {
	pub handler: SubHandler,
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
				match self.start_menu.cursor() {
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
			self.handler.start_count_down(self.count_down);
			global_audio(|audio| audio.play_music());
		} else {
			self.new_game();
		}

		set_played(true);
	}

	fn new_game(&mut self) {
		self.focus.to(Scene::Game);
		self.board.borrow_mut().reset();
		self.next_board.reset();
		self.bag.reset();
		self.stats.reset();
		self.alive_tetromino.set_next(self.bag.next());
		self.next_board.set_next(self.bag.next());
		self.update_ghost_tetromino();
		self.handler.spawn_gravity();
		self.handler.cancel_pause();

		global_audio(|audio| audio.play_music());
	}

	fn handle_game_play(&mut self, event: Event) {
		use TetrominoAction::*;

		if event == Event::CountDown {
			self.count_down -= 1;
			if self.count_down == 0 {
				self.handler.cancel_pause();
				self.handler.spawn_gravity();
				self.check_lock();
			}
			global_audio(|audio| audio.play_sound(Sound::Menu));
			return;
		}

		if self.count_down > 0 {
			return;
		}

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
				if changed {
					self.stats.score += 1;
				}
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
			}
			Event::Z => {
				changed = self.alive_tetromino.rotate(RotateLeft);
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

		let is_move_event =
			matches!(event, Event::Left | Event::Right | Event::Up | Event::Z);

		if changed {
			if is_move_event {
				self.update_ghost_tetromino();
			}
			self.check_lock();
		}

		if is_move_event || event == Event::Down {
			global_audio(|audio| audio.play_sound(Sound::Move));
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
			self.handler.start_lock();
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

		self.ghost_tetromino.kind = self.alive_tetromino.kind;
		self.ghost_tetromino
			.position
			.clone_from(&virtual_tetromino.position);
	}

	fn lock_tetromino(&mut self) {
		if self.alive_tetromino.position.is_outside_the_visible() {
			self.game_over();
			return;
		}

		let cleared_lines = self
			.board
			.borrow_mut()
			.lock_tetromino(&self.alive_tetromino);

		self.alive_tetromino.hidden();
		self.ghost_tetromino.hidden();

		let previous_level = self.stats.level;

		self.stats.update(cleared_lines);

		if self.stats.level > previous_level {
			self.handler.change_level(self.stats.level);
		}

		if cleared_lines != 0 {
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

		global_audio(|audio| {
			audio.stop_music();
			audio.play_sound(Sound::GameOver);
		});
	}

	fn next_alive_tetromino(&mut self) {
		self.alive_tetromino.set_next(self.next_board.current);
		self.next_board.set_next(self.bag.next());
		self.update_ghost_tetromino();
		self.check_lock();

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
				match self.pause_menu.cursor() {
					RESUME => {
						self.focus.back();
						self.handler.cancel_pause();
						self.pause_menu.reset();
					}
					NEW_GAME => {
						self.handler.cancel_lock();
						self.handler.cancel_grvity();
						self.pause_menu.reset();
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
				self.focus.back();
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
			Event::Esc => {
				self.focus.back();
				self.setting_menu.reset();
			}
			_ => (),
		}
	}

	fn handle_game_over_menu(&mut self, event: Event) {
		use game_over_menu_idx::*;

		match event {
			Event::Up => self.game_over_menu.up(),
			Event::Down => self.game_over_menu.down(),
			Event::Enter => {
				match self.game_over_menu.cursor() {
					NEW_GAME => {
						self.new_game();
						self.game_over_menu.reset();
					}
					SCORES => self.focus.push(Scene::Scores),
					QUIT => self.running = false,
					_ => (),
				}
			}
			_ => (),
		}
	}

	pub fn update_line_clear(&mut self) {
		self.board.borrow_mut().particles.update();
		if !self.board.borrow().line_clear.in_progress {
			return;
		}
		if self.board.borrow_mut().update_line_clear() {
			self.next_alive_tetromino();
		}
	}
}
