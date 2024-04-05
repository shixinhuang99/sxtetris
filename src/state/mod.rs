mod bag;
mod tetromino;

use bag::Bag;
use tetromino::Tetromino;
pub use tetromino::TetrominoKind;

use crate::{
	channel::{Event, KeyEvent, Sender},
	consts::{
		MATRIX_X_LEN, MATRIX_Y_LEN, PREVIEW_MATRIX_X_LEN, PREVIEW_MATRIX_Y_LEN,
	},
};

enum TetrominoAction {
	Left,
	Right,
	SoftDrop,
	HardDrop,
}

pub enum CurrentlyScreen {
	StartMenu,
	GameScreen,
}

pub struct State {
	tx: Sender,
	tm_board: [[TetrominoKind; MATRIX_Y_LEN]; MATRIX_X_LEN],
	preview_tm_board:
		[[TetrominoKind; PREVIEW_MATRIX_Y_LEN]; PREVIEW_MATRIX_X_LEN],
	active_tm: Tetromino,
	ghost_tm: Tetromino,
	preview_tm: Tetromino,
	bag: Bag,
	pub running: bool,
	pub paused: bool,
	pub played: bool,
	pub level: u8,
	pub score: u32,
	pub lines: u32,
	pub currently_screen: CurrentlyScreen,
	pub start_menu_selected: usize,
	pub scores: Vec<u32>,
	pub show_scores: bool,
	pub popup_menu_selected: usize,
	lock_start: bool,
}

impl State {
	pub fn new(tx: Sender) -> Self {
		let mut bag = Bag::new();

		Self {
			tx,
			tm_board: [[TetrominoKind::None; MATRIX_Y_LEN]; MATRIX_X_LEN],
			preview_tm_board: [[TetrominoKind::None; PREVIEW_MATRIX_Y_LEN];
				PREVIEW_MATRIX_X_LEN],
			active_tm: Tetromino::default(),
			ghost_tm: Tetromino::new(TetrominoKind::Ghost),
			preview_tm: Tetromino::new_without_offest(bag.next()),
			bag,
			running: true,
			paused: true,
			played: false,
			level: u8::MAX,
			score: u32::MAX,
			lines: u32::MAX,
			currently_screen: CurrentlyScreen::StartMenu,
			start_menu_selected: 0,
			scores: vec![u32::MAX; 10],
			show_scores: false,
			popup_menu_selected: 0,
			lock_start: false,
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		match self.currently_screen {
			CurrentlyScreen::StartMenu => {
				if let Event::Key(key) = event {
					self.operate_start_menu(key);
				}
			}
			CurrentlyScreen::GameScreen => {
				self.operate_game(event);
			}
		}
	}

	fn operate_game(&mut self, event: Event) {
		match event {
			Event::Key(key) => {
				if self.paused {
					self.operate_popup_menu(key);
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
				self.next_active_tm();
			}
			_ => (),
		};
	}

	fn operate_start_menu(&mut self, key: KeyEvent) {
		let end_idx = if self.played {
			3
		} else {
			2
		};
		match key {
			KeyEvent::Up => {
				if self.start_menu_selected == 0 {
					self.start_menu_selected = end_idx;
				} else {
					self.start_menu_selected -= 1;
				}
			}
			KeyEvent::Down => {
				if self.start_menu_selected == end_idx {
					self.start_menu_selected = 0;
				} else {
					self.start_menu_selected += 1;
				}
			}
			KeyEvent::Enter | KeyEvent::Space => {
				let offest = if self.played {
					0
				} else {
					1
				};
				match self.start_menu_selected + offest {
					menu_mapping::CONTINUE => {
						self.currently_screen = CurrentlyScreen::GameScreen;
					}
					menu_mapping::NEW_GAME => {
						self.new_game();
					}
					menu_mapping::SCORES => {
						self.show_scores = true;
					}
					menu_mapping::QUIT => {
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

	fn operate_popup_menu(&mut self, key: KeyEvent) {
		match key {
			KeyEvent::Up => {
				if self.popup_menu_selected == 0 {
					self.popup_menu_selected = 3;
				} else {
					self.popup_menu_selected -= 1;
				}
			}
			KeyEvent::Down => {
				if self.popup_menu_selected == 3 {
					self.popup_menu_selected = 0;
				} else {
					self.popup_menu_selected += 1;
				}
			}
			KeyEvent::Enter | KeyEvent::Space => {
				match self.popup_menu_selected {
					menu_mapping::CONTINUE => {
						self.cancel_pause();
					}
					menu_mapping::NEW_GAME => {
						self.new_game();
					}
					menu_mapping::SCORES => {
						self.show_scores = true;
					}
					menu_mapping::QUIT => {
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
					self.popup_menu_selected = 0;
				}
			}
			_ => (),
		}
	}

	pub fn new_game(&mut self) {
		let tx = self.tx.clone();
		*self = Self::new(tx);
		self.next_active_tm();
		self.paused = false;
		self.send(Event::AutoDropStart);
		self.played = true;
		self.currently_screen = CurrentlyScreen::GameScreen;
	}

	pub fn tm_board_mapping(&self, x: usize, y: usize) -> &TetrominoKind {
		&self.tm_board[x][y]
	}

	pub fn perview_tm_board_mapping(
		&self,
		x: usize,
		y: usize,
	) -> &TetrominoKind {
		&self.preview_tm_board[x][y]
	}

	fn move_active_tm(&mut self, tm_action: TetrominoAction) {
		if let TetrominoKind::None = self.active_tm.kind {
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
			if let TetrominoAction::HardDrop = tm_action {
				self.next_active_tm();
			} else {
				self.lock_start = true;
				self.send(Event::LockReset);
			}
		}
	}

	fn update_active_tm(&mut self, tm: Tetromino) {
		for p in &self.active_tm.pos {
			self.tm_board[p.0][p.1] = TetrominoKind::None;
		}

		self.active_tm = tm;

		for p in &self.active_tm.pos {
			self.tm_board[p.0][p.1] = self.active_tm.kind;
		}
	}

	fn next_active_tm(&mut self) {
		for pos in &self.preview_tm.pos {
			self.preview_tm_board[pos.0][pos.1] = TetrominoKind::None;
		}

		self.active_tm = Tetromino::new(self.preview_tm.kind);
		self.preview_tm = Tetromino::new_without_offest(self.bag.next());

		for pos in &self.active_tm.pos {
			self.tm_board[pos.0][pos.1] = self.active_tm.kind;
		}

		for pos in &self.preview_tm.pos {
			self.preview_tm_board[pos.0][pos.1] = self.preview_tm.kind;
		}

		self.move_ghost_tm();
	}

	fn update_ghost_tm(&mut self, tm: Tetromino) {
		for pos in &self.ghost_tm.pos {
			let kind = &mut self.tm_board[pos.0][pos.1];
			if !matches!(kind, TetrominoKind::None | TetrominoKind::Ghost) {
				continue;
			}
			*kind = TetrominoKind::None;
		}

		self.ghost_tm = tm;

		for p in &self.ghost_tm.pos {
			self.tm_board[p.0][p.1] = TetrominoKind::Ghost;
		}
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
		if let TetrominoAction::HardDrop = tm_action {
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
		tm.pos.iter().any(|pos| {
			if self.active_tm.pos.iter().any(|l_pos| pos == l_pos) {
				return false;
			}
			!matches!(
				&self.tm_board[pos.0][pos.1],
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

mod menu_mapping {
	pub const CONTINUE: usize = 0;
	pub const NEW_GAME: usize = 1;
	pub const SCORES: usize = 2;
	pub const QUIT: usize = 3;
}
