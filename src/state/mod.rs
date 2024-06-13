mod bag;
mod board;
mod consts;
mod list;
mod position;
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
	lock_start: bool,
	pub start_menu: ListState,
	pub pause_menu: ListState,
}

impl State {
	pub fn new(tx: Sender) -> Self {
		let mut bag = Bag::new();

		let state = Self {
			tx,
			board: BoardState::new(BOARD_Y_LEN, BOARD_X_LEN),
			preview_board: BoardState::new(
				PREVIEW_BOARD_Y_LEN,
				PREVIEW_BOARD_X_LEN,
			),
			active_tm: Tetromino::default(),
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
			lock_start: false,
			start_menu: ListState::new(&START_MENU_ITEMS),
			pause_menu: ListState::new(&PAUSE_MENU_ITEMS),
		};

		state
	}

	pub fn new_game(&mut self) {
		*self = Self::new(self.tx.clone());
		self.gen_next_active_tm();
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
						self.move_active_tm(TetrominoAction::Left);
					}
					KeyEvent::Right => {
						self.move_active_tm(TetrominoAction::Right);
					}
					KeyEvent::Down => {
						self.move_active_tm(TetrominoAction::SoftDrop);
					}
					KeyEvent::Space => {
						self.move_active_tm(TetrominoAction::HardDrop);
					}
					KeyEvent::Esc | KeyEvent::P => {
						self.pause();
					}
					_ => (),
				}
			}
			Event::AutoDrop => {
				self.move_active_tm(TetrominoAction::SoftDrop);
			}
			Event::FocusLost => {
				self.pause();
			}
			Event::LockEnd => {
				self.lock_start = false;
				self.gen_next_active_tm();
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

	fn move_active_tm(&mut self, tm_action: TetrominoAction) {
		if self.active_tm.kind == TetrominoKind::None {
			return;
		}

		let Some(next_tetromino) = self.calc_next_position(&tm_action) else {
			return;
		};

		self.update_active_tm(next_tetromino);

		if matches!(tm_action, TetrominoAction::Left | TetrominoAction::Right) {
			if self.lock_start {
				self.send(Event::LockReset);
			} else {
				self.move_ghost_tm();
			}
		}

		if self.active_tm.same_position(&self.ghost_tm) {
			if tm_action == TetrominoAction::HardDrop {
				self.gen_next_active_tm();
			} else {
				self.lock_start = true;
				self.send(Event::LockReset);
			}
		}
	}

	fn update_active_tm(&mut self, tm: Tetromino) {
		self.board.clear_area(&self.active_tm.position);
		self.board.update_area(&tm.position, tm.kind);
		self.active_tm = tm;
	}

	fn gen_next_active_tm(&mut self) {
		self.preview_board.clear_area(&self.preview_tm.position);

		let active_tm = Tetromino::new(self.preview_tm.kind);
		let preview_tm = Tetromino::new_preview(self.bag.next());

		self.board.update_area(&active_tm.position, active_tm.kind);
		self.preview_board
			.update_area(&preview_tm.position, preview_tm.kind);

		self.active_tm = active_tm;
		self.preview_tm = preview_tm;

		self.move_ghost_tm();
	}

	fn update_ghost_tm(&mut self, tm: Tetromino) {
		self.board.clear_area_if(&self.ghost_tm.position, |kind| {
			matches!(kind, TetrominoKind::None | TetrominoKind::Ghost)
		});
		self.board.update_area(&tm.position, TetrominoKind::Ghost);
		self.ghost_tm = tm;
	}

	fn move_ghost_tm(&mut self) {
		let mut virtual_tm = self.active_tm.clone();

		loop {
			if virtual_tm.down() {
				self.update_ghost_tm(virtual_tm);
				break;
			};

			if self.is_collision(&virtual_tm) {
				virtual_tm.up();
				self.update_ghost_tm(virtual_tm);
				break;
			}
		}
	}

	fn calc_next_position(
		&mut self,
		tm_action: &TetrominoAction,
	) -> Option<Tetromino> {
		if *tm_action == TetrominoAction::HardDrop {
			return Some(self.ghost_tm.clone());
		}

		let mut virtual_tm = self.active_tm.clone();

		let touch_border = match tm_action {
			TetrominoAction::SoftDrop => virtual_tm.down(),
			TetrominoAction::Left => virtual_tm.left(),
			TetrominoAction::Right => virtual_tm.right(),
			_ => unreachable!(),
		};

		if touch_border || self.is_collision(&virtual_tm) {
			None
		} else {
			Some(virtual_tm)
		}
	}

	fn is_collision(&self, tm: &Tetromino) -> bool {
		tm.position.points.iter().any(|p| {
			if self
				.active_tm
				.position
				.points
				.iter()
				.any(|other| p.0 == other.0 && p.1 == other.1)
			{
				return false;
			}
			!matches!(
				self.board.get_cell(p.0, p.1),
				TetrominoKind::None | TetrominoKind::Ghost
			)
		})
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
		self.send(Event::CancelPause);
	}
}
