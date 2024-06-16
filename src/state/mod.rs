mod bag;
mod board;
mod consts;
mod list;
mod point;
mod tetromino;

use bag::Bag;
pub use board::BoardState;
use consts::{
	game_over_menu_idx, pause_menu_idx, start_menu_idx, GAME_OVER_MENU_ITEMS,
	PAUSE_MENU_ITEMS, START_MENU_ITEMS,
};
pub use list::ListState;
pub use tetromino::TetrominoKind;
use tetromino::{Tetromino, TetrominoAction};

use crate::{
	channel::{Event, KeyEvent, Sender},
	consts::{
		BOARD_X_LEN, BOARD_Y_LEN, PREVIEW_BOARD_X_LEN, PREVIEW_BOARD_Y_LEN,
	},
};

pub enum CurrentlyScreen {
	StartMenu,
	Game,
}

pub struct State {
	tx: Sender,
	pub board: BoardState,
	pub preview_board: BoardState,
	active_tm: Tetromino,
	ghost_tm: Tetromino,
	preview_tm: Tetromino,
	bag: Bag,
	pub running: bool,
	pub paused: bool,
	pub level: u32,
	pub score: u32,
	pub lines: u32,
	pub currently_screen: CurrentlyScreen,
	pub scores: Vec<u32>,
	pub show_scores: bool,
	tm_lock_started: bool,
	pub start_menu: ListState,
	pub pause_menu: ListState,
	pub game_over: bool,
	pub game_over_menu: ListState,
	combo: i32,
}

impl State {
	pub fn new(tx: Sender) -> Self {
		let mut bag = Bag::new();

		Self {
			tx,
			board: BoardState::new(BOARD_Y_LEN, BOARD_X_LEN),
			preview_board: BoardState::new(
				PREVIEW_BOARD_Y_LEN,
				PREVIEW_BOARD_X_LEN,
			),
			active_tm: Tetromino::new(TetrominoKind::None),
			ghost_tm: Tetromino::new(TetrominoKind::Ghost),
			preview_tm: Tetromino::new_preview(bag.next()),
			bag,
			running: true,
			paused: true,
			level: 1,
			score: 0,
			lines: 0,
			currently_screen: CurrentlyScreen::StartMenu,
			scores: vec![0; 10],
			show_scores: false,
			tm_lock_started: false,
			start_menu: ListState::new(&START_MENU_ITEMS),
			pause_menu: ListState::new(&PAUSE_MENU_ITEMS),
			game_over: false,
			game_over_menu: ListState::new(&GAME_OVER_MENU_ITEMS),
			combo: -1,
		}
	}

	pub fn new_game(&mut self) {
		let tx = self.tx.clone();
		let scores = self.scores.clone();

		*self = Self::new(tx);

		self.scores = scores;
		self.send(Event::GravityReset);
		self.currently_screen = CurrentlyScreen::Game;
		self.gen_next_tm();
		self.cancel_pause();
	}

	fn gen_next_tm(&mut self) {
		self.reset_lock();
		self.send(Event::GravityReset);

		let pre_level = self.level;
		let lines = self.board.check_and_clear_line();
		self.update_stats(lines);
		if self.level != pre_level {
			self.send(Event::LevelChange(self.level));
		}

		self.active_tm = Tetromino::new(self.preview_tm.kind);
		self.game_over = self.board.is_collision(&self.active_tm.points);
		self.move_ghost_tm();
		self.board.update_area(&self.active_tm);
		self.preview_board.clear_area(&self.preview_tm);
		self.preview_tm = Tetromino::new_preview(self.bag.next());
		self.preview_board.update_area(&self.preview_tm);

		if self.game_over {
			self.scores.push(self.score);
			self.scores.sort_unstable_by(|a, b| b.cmp(a));
			self.scores.truncate(10);
			self.pause();
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		match self.currently_screen {
			CurrentlyScreen::StartMenu => {
				if let Event::Key(key) = event {
					self.update_start_menu(key);
				}
			}
			CurrentlyScreen::Game => {
				self.update_game(event);
			}
		}
	}

	fn update_game(&mut self, event: Event) {
		match event {
			Event::Key(key) => {
				if self.game_over {
					self.update_game_over_menu(key);
					return;
				} else if self.paused {
					self.update_pause_menu(key);
					return;
				}
				match key {
					KeyEvent::Left => {
						self.move_tm(TetrominoAction::Left);
					}
					KeyEvent::Right => {
						self.move_tm(TetrominoAction::Right);
					}
					KeyEvent::Down => {
						self.move_tm(TetrominoAction::SoftDrop);
					}
					KeyEvent::Space => {
						self.move_tm(TetrominoAction::HardDrop);
					}
					KeyEvent::Up => {
						self.rotate_tm(TetrominoAction::RotateRight);
					}
					KeyEvent::Z => {
						self.rotate_tm(TetrominoAction::RotateLeft);
					}
					KeyEvent::Esc | KeyEvent::P => {
						self.pause();
					}
					_ => (),
				}
			}
			Event::Gravity => {
				self.move_tm(TetrominoAction::SoftDrop);
			}
			Event::FocusLost => {
				self.pause();
			}
			Event::LockEnd => {
				self.gen_next_tm();
			}
			_ => (),
		};
	}

	fn update_start_menu(&mut self, key: KeyEvent) {
		use start_menu_idx::*;

		match key {
			KeyEvent::Up => {
				self.start_menu.up();
			}
			KeyEvent::Down => {
				self.start_menu.down();
			}
			KeyEvent::Enter => {
				match self.start_menu.cursor {
					PLAY => {
						self.new_game();
					}
					SCORES => {
						self.show_scores = true;
					}
					QUIT => {
						self.running = false;
					}
					_ => (),
				}
			}
			KeyEvent::Esc => {
				if self.show_scores {
					self.show_scores = false;
				} else {
					self.running = false;
				}
			}
			_ => (),
		}
	}

	fn update_pause_menu(&mut self, key: KeyEvent) {
		use pause_menu_idx::*;

		match key {
			KeyEvent::Up => {
				self.pause_menu.up();
			}
			KeyEvent::Down => {
				self.pause_menu.down();
			}
			KeyEvent::Enter => {
				match self.pause_menu.cursor {
					RESUME => {
						self.cancel_pause();
					}
					NEW_GAME => {
						self.new_game();
					}
					SCORES => {
						self.show_scores = true;
					}
					QUIT => {
						self.running = false;
					}
					_ => (),
				}
			}
			KeyEvent::Esc | KeyEvent::P => {
				if self.show_scores {
					self.show_scores = false;
				} else {
					self.cancel_pause();
					self.pause_menu.reset();
				}
			}
			_ => (),
		}
	}

	fn move_tm(&mut self, tm_action: TetrominoAction) {
		let next_tm = if tm_action == TetrominoAction::HardDrop {
			let mut next = self.ghost_tm.clone();
			next.kind = self.active_tm.kind;

			Some(next)
		} else {
			let mut virtual_tm = self.active_tm.clone();

			if !virtual_tm.walk(&tm_action)
				|| self.is_collision_ignore_self(&virtual_tm)
			{
				None
			} else {
				Some(virtual_tm)
			}
		};

		if let Some(tm) = next_tm {
			let distance =
				tm.points.value[0].1 - self.active_tm.points.value[0].1;

			self.board.clear_area(&self.active_tm);
			self.active_tm = tm;
			if matches!(
				tm_action,
				TetrominoAction::Left
					| TetrominoAction::Right
					| TetrominoAction::RotateLeft
					| TetrominoAction::RotateRight
			) {
				self.move_ghost_tm();
			};
			self.board.update_area(&self.active_tm);

			if tm_action == TetrominoAction::SoftDrop {
				self.score += 1;
			}

			if tm_action == TetrominoAction::HardDrop {
				self.gen_next_tm();
				self.score += distance as u32 * 2;
				return;
			}

			self.update_tm_lock();
		}
	}

	fn update_tm_lock(&mut self) {
		let fit_together = self.active_tm.same_position(&self.ghost_tm);
		if self.tm_lock_started {
			if !fit_together {
				self.reset_lock();
			} else {
				self.send(Event::LockRefresh);
			}
		} else if fit_together {
			self.tm_lock_started = true;
			self.send(Event::LockRefresh);
		}
	}

	fn move_ghost_tm(&mut self) {
		let mut virtual_tm = self.active_tm.clone();

		let bottom_point =
			virtual_tm.points.value.iter().max_by(|a, b| a.1.cmp(&b.1));

		if let Some(point) = bottom_point {
			let mut distance = BOARD_Y_LEN as i32 - point.1 - 1;
			let mut points = virtual_tm.points.clone();
			while distance > 0 {
				if !virtual_tm.walk(&TetrominoAction::SoftDrop)
					|| self.is_collision_ignore_self(&virtual_tm)
				{
					break;
				}
				points = virtual_tm.points.clone();
				distance -= 1;
			}
			virtual_tm.points = points;
			self.board.clear_area_if(&self.ghost_tm, |kind| {
				*kind == TetrominoKind::Ghost
			});
			self.ghost_tm = virtual_tm;
			self.ghost_tm.kind = TetrominoKind::Ghost;
			self.board.update_area(&self.ghost_tm);
		}
	}

	fn is_collision_ignore_self(&self, tm: &Tetromino) -> bool {
		self.board
			.is_collision_with_ignore(&tm.points, &self.active_tm.points)
	}

	fn send(&self, event: Event) {
		self.tx.send(event).unwrap();
	}

	fn pause(&mut self) {
		self.paused = true;
		self.send(Event::Pause);
	}

	fn cancel_pause(&mut self) {
		self.paused = false;
		self.send(Event::PauseCancel);
	}

	fn rotate_tm(&mut self, tm_action: TetrominoAction) {
		let mut virtual_tm = self.active_tm.clone();

		if virtual_tm.rotate(&tm_action, |points| {
			self.board
				.is_collision_with_ignore(points, &self.active_tm.points)
		}) {
			self.board.clear_area(&self.active_tm);
			self.active_tm = virtual_tm;
			self.move_ghost_tm();
			self.board.update_area(&self.active_tm);
			self.update_tm_lock();
		}
	}

	fn update_game_over_menu(&mut self, key: KeyEvent) {
		use game_over_menu_idx::*;

		match key {
			KeyEvent::Up => {
				self.game_over_menu.up();
			}
			KeyEvent::Down => {
				self.game_over_menu.down();
			}
			KeyEvent::Enter => {
				match self.game_over_menu.cursor {
					NEW_GAME => {
						self.new_game();
					}
					SCORES => {
						self.show_scores = true;
					}
					QUIT => {
						self.running = false;
					}
					_ => (),
				}
			}
			KeyEvent::Esc | KeyEvent::P => {
				if self.show_scores {
					self.show_scores = false;
				}
			}
			_ => (),
		}
	}

	fn update_stats(&mut self, lines: u32) {
		self.lines += lines;
		self.level = self.lines / 10 + 1;
		let base_score = match lines {
			1 => 100,
			2 => 300,
			3 => 500,
			4 => 800,
			_ => 0,
		};
		self.score += base_score * self.level;
		if lines > 0 {
			self.combo += 1;
		} else {
			self.combo = -1;
		}
		if self.combo > 0 {
			self.score += 50 * self.combo as u32 * self.level;
		}
	}

	fn reset_lock(&mut self) {
		self.tm_lock_started = false;
		self.send(Event::LockReset);
	}
}
