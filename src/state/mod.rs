mod bag;
mod board;
mod consts;
mod list;
mod point;
mod tetromino;

use bag::Bag;
pub use board::BoardState;
use consts::{
	pause_menu_idx, start_menu_idx, PAUSE_MENU_ITEMS, START_MENU_ITEMS,
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
	pub level: u8,
	pub score: u32,
	pub lines: u32,
	pub currently_screen: CurrentlyScreen,
	pub scores: Vec<u32>,
	pub show_scores: bool,
	tm_lock_started: bool,
	pub start_menu: ListState,
	pub pause_menu: ListState,
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
			level: u8::MAX,
			score: u32::MAX,
			lines: u32::MAX,
			currently_screen: CurrentlyScreen::StartMenu,
			scores: vec![u32::MAX; 10],
			show_scores: false,
			tm_lock_started: false,
			start_menu: ListState::new(&START_MENU_ITEMS),
			pause_menu: ListState::new(&PAUSE_MENU_ITEMS),
		}
	}

	pub fn new_game(&mut self) {
		*self = Self::new(self.tx.clone());
		self.gen_next_tm();
		self.paused = false;
		self.send(Event::AutoDropStart);
		self.currently_screen = CurrentlyScreen::Game;
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
				if self.paused {
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
			Event::AutoDrop => {
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
			KeyEvent::Enter | KeyEvent::Space => {
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
			KeyEvent::Enter | KeyEvent::Space => {
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

			if !virtual_tm.walk(&tm_action) || self.is_collision(&virtual_tm) {
				None
			} else {
				Some(virtual_tm)
			}
		};

		if let Some(tm) = next_tm {
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

			if tm_action == TetrominoAction::HardDrop {
				self.gen_next_tm();
				return;
			}

			self.update_tm_lock();
		}
	}

	fn update_tm_lock(&mut self) {
		if self.tm_lock_started {
			self.send(Event::LockRefresh);
		} else if self.active_tm.same_position(&self.ghost_tm) {
			self.tm_lock_started = true;
			self.send(Event::LockRefresh);
		}
	}

	fn gen_next_tm(&mut self) {
		self.tm_lock_started = false;
		self.send(Event::LockReset);
		self.active_tm = Tetromino::new(self.preview_tm.kind);
		self.move_ghost_tm();
		self.board.update_area(&self.active_tm);
		self.preview_board.clear_area(&self.preview_tm);
		self.preview_tm = Tetromino::new_preview(self.bag.next());
		self.preview_board.update_area(&self.preview_tm);
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
					|| self.is_collision(&virtual_tm)
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

	fn is_collision(&self, tm: &Tetromino) -> bool {
		self.board.is_collision(&tm.points, &self.active_tm.points)
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
		if self.active_tm.kind == TetrominoKind::O {
			return;
		}

		let mut virtual_tm = self.active_tm.clone();

		if virtual_tm.rotate(&tm_action, |points| {
			self.board.is_collision(points, &self.active_tm.points)
		}) {
			self.board.clear_area(&self.active_tm);
			self.active_tm = virtual_tm;
			self.move_ghost_tm();
			self.board.update_area(&self.active_tm);
			self.update_tm_lock();
		}
	}
}
