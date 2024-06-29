mod bag;
mod board;
mod consts;
mod list;
mod point;
mod tetromino;
mod tetromino_type;

use bag::Bag;
pub use board::BoardState;
use consts::{
	game_over_menu_idx, pause_menu_idx, start_menu_idx, GAME_OVER_MENU_ITEMS,
	PAUSE_MENU_ITEMS, START_MENU_ITEMS,
};
pub use list::ListState;
use point::Points;
use tetromino::{Tetromino, TetrominoAction};
pub use tetromino_type::TetrominoType;

use crate::{
	animation::confetti::{ConfettiDirection, ConfettiState},
	consts::{BOARD_COLS, BOARD_ROWS, PREVIEW_BOARD_COLS, PREVIEW_BOARD_ROWS},
	handler::{is_locked, is_paused, GameEvent, SubHandler},
	save::Save,
};

const BOARD_ROWS_I32: i32 = BOARD_ROWS as i32;

#[derive(PartialEq)]
pub enum Screen {
	StartMenu,
	Game,
}

pub struct State {
	pub handler: SubHandler,
	pub running: bool,
	pub screen: Screen,
	pub start_menu: ListState,
	pub pause_menu: ListState,
	pub game_over_menu: ListState,
	pub bag: Bag,
	pub board: BoardState,
	pub preview_board: BoardState,
	pub active_tm: Tetromino,
	ghost_tm: Tetromino,
	pub preview_tm: Tetromino,
	pub level: u32,
	pub score: u32,
	pub lines: u32,
	pub combo: i32,
	pub scores: Vec<u32>,
	pub show_scores: bool,
	pub is_game_over: bool,
	pub count_down: u8,
	pub blinking: bool,
	pub show_help: bool,
	pub show_about: bool,
	pub confetti_state: ConfettiState,
}

impl State {
	pub fn new(handler: SubHandler) -> Self {
		Self {
			handler,
			running: true,
			screen: Screen::StartMenu,
			start_menu: ListState::new(&START_MENU_ITEMS),
			pause_menu: ListState::new(&PAUSE_MENU_ITEMS),
			game_over_menu: ListState::new(&GAME_OVER_MENU_ITEMS),
			board: BoardState::new(BOARD_ROWS, BOARD_COLS),
			preview_board: BoardState::new(
				PREVIEW_BOARD_ROWS,
				PREVIEW_BOARD_COLS,
			),
			bag: Bag::new(),
			active_tm: Tetromino::new(TetrominoType::None),
			preview_tm: Tetromino::new_preview(TetrominoType::None),
			ghost_tm: Tetromino::new(TetrominoType::Ghost),
			level: 1,
			score: 0,
			lines: 0,
			combo: -1,
			scores: vec![0; 10],
			show_scores: false,
			is_game_over: false,
			count_down: 0,
			blinking: false,
			show_help: false,
			show_about: false,
			confetti_state: ConfettiState::new(),
		}
	}

	pub fn receive_event(&mut self, event: GameEvent) {
		if let GameEvent::CountDown(v) = event {
			self.count_down = v;
			if self.count_down == 0 {
				self.handler.cancel_pause();
				self.handler.spawn_gravity_task();
				self.check_lock();
			}
			return;
		}
		match self.screen {
			Screen::StartMenu => {
				self.update_start_menu(event);
			}
			Screen::Game => {
				if self.is_game_over {
					self.update_game_over_menu(event);
				} else if is_paused() {
					if self.count_down > 0 {
						return;
					}
					self.update_pause_menu(event);
				} else {
					self.update_game(event);
				}
			}
		}
	}

	fn update_game(&mut self, event: GameEvent) {
		match event {
			GameEvent::Left => {
				self.move_tm(TetrominoAction::Left);
			}
			GameEvent::Right => {
				self.move_tm(TetrominoAction::Right);
			}
			GameEvent::Down => {
				self.move_tm(TetrominoAction::SoftDrop);
			}
			GameEvent::Space => {
				self.move_tm(TetrominoAction::HardDrop);
			}
			GameEvent::Up => {
				self.rotate_tm(TetrominoAction::RotateRight);
			}
			GameEvent::Z => {
				self.rotate_tm(TetrominoAction::RotateLeft);
			}
			GameEvent::Esc | GameEvent::P | GameEvent::FocusLost => {
				self.handler.pause();
			}
			GameEvent::Gravity => {
				self.move_tm(TetrominoAction::SoftDrop);
			}
			GameEvent::LockEnd => {
				self.gen_next_tm();
			}
			GameEvent::Blink => {
				self.blinking = !self.blinking;
			}
			_ => (),
		};
	}

	fn update_start_menu(&mut self, event: GameEvent) {
		use start_menu_idx::*;

		match event {
			GameEvent::Up => {
				self.start_menu.up();
			}
			GameEvent::Down => {
				self.start_menu.down();
			}
			GameEvent::Enter => {
				match self.start_menu.cursor {
					PLAY => {
						self.play();
					}
					SCORES => {
						self.show_scores = true;
					}
					HELP => {
						self.show_help = true;
					}
					ABOUT => {
						self.show_about = true;
					}
					QUIT => {
						self.running = false;
					}
					_ => (),
				}
			}
			GameEvent::Esc => {
				if self.show_scores {
					self.show_scores = false;
				} else if self.show_help {
					self.show_help = false;
				} else if self.show_about {
					self.show_about = false;
				} else {
					self.running = false;
				}
			}
			_ => (),
		}
	}

	fn update_pause_menu(&mut self, event: GameEvent) {
		use pause_menu_idx::*;

		match event {
			GameEvent::Up => {
				self.pause_menu.up();
			}
			GameEvent::Down => {
				self.pause_menu.down();
			}
			GameEvent::Enter => {
				match self.pause_menu.cursor {
					RESUME => {
						self.handler.cancel_pause();
					}
					NEW_GAME => {
						self.handler.cancel_lock();
						self.handler.cancel_grvity();
						self.handler.cancel_pause();
						self.new_game();
					}
					SCORES => {
						self.show_scores = true;
					}
					HELP => {
						self.show_help = true;
					}
					QUIT => {
						self.running = false;
					}
					_ => (),
				}
			}
			GameEvent::Esc | GameEvent::P => {
				if self.show_scores {
					self.show_scores = false;
				} else if self.show_help {
					self.show_help = false;
				} else {
					self.handler.cancel_pause();
					self.pause_menu.reset();
				}
			}
			_ => (),
		}
	}

	fn update_game_over_menu(&mut self, event: GameEvent) {
		use game_over_menu_idx::*;

		match event {
			GameEvent::Up => {
				self.game_over_menu.up();
			}
			GameEvent::Down => {
				self.game_over_menu.down();
			}
			GameEvent::Enter => {
				match self.game_over_menu.cursor {
					NEW_GAME => {
						self.handler.cancel_pause();
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
			GameEvent::Esc | GameEvent::P => {
				if self.show_scores {
					self.show_scores = false;
				}
			}
			_ => (),
		}
	}

	fn play(&mut self) {
		if self.count_down > 0 {
			self.screen = Screen::Game;
			self.update_ghost_tm();
			self.board.update_area(&self.active_tm);
			self.preview_board.update_area(&self.preview_tm);
			self.handler.pause();
			self.handler.spawn_count_down_task(self.count_down);
		} else {
			self.new_game();
		}
	}

	fn new_game(&mut self) {
		self.board.reset();
		self.preview_board.reset();
		self.bag.reset();
		self.level = 1;
		self.score = 0;
		self.lines = 0;
		self.combo = -1;
		self.pause_menu.reset();
		self.game_over_menu.reset();
		self.is_game_over = false;
		self.blinking = false;
		self.screen = Screen::Game;
		self.confetti_state.reset();

		self.active_tm = Tetromino::new(self.bag.next());
		self.update_ghost_tm();
		self.board.update_area(&self.active_tm);
		self.preview_tm = Tetromino::new_preview(self.bag.next());
		self.preview_board.update_area(&self.preview_tm);

		self.handler.spawn_gravity_task();
		self.handler.cancel_pause();
	}

	fn game_over(&mut self) {
		self.is_game_over = true;
		self.handler.pause();
		self.handler.cancel_grvity();
		self.handler.cancel_lock();
		self.scores.push(self.score);
		self.scores.sort_unstable_by(|a, b| b.cmp(a));
		self.scores.truncate(10);
	}

	fn gen_next_tm(&mut self) {
		self.push_tm_points_to_confetti();
		self.update_stats();

		if self.active_tm.points.is_out_of_visible_arae() {
			self.game_over();
			return;
		}

		self.active_tm = Tetromino::new(self.preview_tm.tm_type);

		if self.board.is_collision(&self.active_tm.points) {
			self.game_over();
			self.blinking = false;
			self.board.update_area(&self.active_tm);
			return;
		}

		self.blinking = false;
		self.update_ghost_tm();
		self.board.update_area(&self.active_tm);
		self.preview_board.clear_area(&self.preview_tm);
		self.preview_tm = Tetromino::new_preview(self.bag.next());
		self.preview_board.update_area(&self.preview_tm);

		self.handler.reset_gravity();
	}

	fn move_tm(&mut self, tm_action: TetrominoAction) {
		let next_points = if tm_action == TetrominoAction::HardDrop {
			Some(self.ghost_tm.points.clone())
		} else {
			self.active_tm.can_move(&tm_action, |points| {
				self.is_collision_ignore_self(points)
			})
		};

		if let Some(points) = next_points {
			let distance = points.value[0].1 - self.active_tm.points.value[0].1;

			self.board.clear_area(&self.active_tm);
			self.active_tm.points = points;
			if matches!(
				tm_action,
				TetrominoAction::Left
					| TetrominoAction::Right
					| TetrominoAction::RotateLeft
					| TetrominoAction::RotateRight
			) {
				self.update_ghost_tm();
			};
			self.board.update_area(&self.active_tm);

			if tm_action == TetrominoAction::SoftDrop {
				self.score += 1;
			}

			if tm_action == TetrominoAction::HardDrop {
				self.score += distance as u32 * 2;
				self.handler.cancel_lock();
				self.gen_next_tm();
				return;
			}

			self.refresh_lock();

			return;
		}

		self.check_lock();
	}

	fn rotate_tm(&mut self, tm_action: TetrominoAction) {
		if let Some((next_points, next_deg)) = self.active_tm.can_rotate(
			&tm_action,
			&self.active_tm.points,
			|points, ignore| {
				self.board.is_collision_with_ignore(points, ignore)
			},
		) {
			self.board.clear_area(&self.active_tm);
			self.active_tm.points = next_points;
			self.active_tm.rotate_deg = next_deg;
			self.update_ghost_tm();
			self.board.update_area(&self.active_tm);
			self.refresh_lock();
			return;
		}

		self.check_lock();
	}

	fn check_lock(&mut self) {
		if !self.active_tm.same_position(&self.ghost_tm) || is_locked() {
			return;
		}
		self.handler.spawn_lock_task();
	}

	fn refresh_lock(&mut self) {
		let fit_together = self.active_tm.same_position(&self.ghost_tm);
		if is_locked() {
			if !fit_together {
				self.blinking = false;
				self.handler.cancel_lock();
			} else {
				self.handler.refresh_lock();
			}
		} else if fit_together {
			self.handler.spawn_lock_task();
		}
	}

	// must call before update active tetromino area
	fn update_ghost_tm(&mut self) {
		if let Some(point) = self.active_tm.points.bottom_point() {
			let mut virtual_tm = self.active_tm.clone();
			let mut distance = BOARD_ROWS_I32 - point.1 - 1;

			while distance > 0 {
				if let Some(next_points) = virtual_tm
					.can_move(&TetrominoAction::SoftDrop, |points| {
						self.is_collision_ignore_self(points)
					}) {
					virtual_tm.points = next_points;
					distance -= 1;
				} else {
					break;
				}
			}

			self.board.clear_area_if(&self.ghost_tm, |tm_type| {
				*tm_type == TetrominoType::Ghost
			});
			self.ghost_tm = virtual_tm;
			self.ghost_tm.tm_type = TetrominoType::Ghost;
			self.board.update_area(&self.ghost_tm);
		}
	}

	fn is_collision_ignore_self(&self, points: &Points) -> bool {
		self.board
			.is_collision_with_ignore(points, &self.active_tm.points)
	}

	fn update_stats(&mut self) {
		let previous_level = self.level;

		let lines = self.board.clear_line();

		if lines > 0 {
			self.lines += lines;

			let new_level = self.lines / 10 + 1;

			if new_level > previous_level {
				self.level = new_level;
				self.handler.change_level(self.level);
			}

			let base_score = match lines {
				1 => 100,
				2 => 300,
				3 => 500,
				4 => 800,
				_ => 0,
			};
			self.score += base_score * self.level;
			self.combo += 1;
		} else {
			self.combo = -1;
		}
		if self.combo > 0 {
			self.score += 50 * self.combo as u32 * self.level;
		}
	}

	pub fn read_save(&mut self, save: &Save) {
		self.scores.clone_from(&save.scores);
		if let Some(last_game) = &save.last_game {
			self.count_down = 3;
			self.board.deserialize(&last_game.board);
			self.bag.deserialize(&last_game.bag);
			self.active_tm.deserialize(&last_game.active_tm);
			self.preview_tm.deserialize(&last_game.preview_tm);
			self.level = last_game.level;
			self.score = last_game.score;
			self.lines = last_game.lines;
			self.combo = last_game.combo;
		}
	}

	pub fn push_tm_points_to_confetti(&mut self) {
		use ConfettiDirection::*;

		const DIRS: [ConfettiDirection; 4] = [Top, Right, Bottom, Left];
		let points = self.active_tm.points.usize_points();

		for p in points.iter() {
			for dir in DIRS {
				match dir {
					Top => {
						if p.1 == 0 {
							continue;
						}
						let y = p.1 - 1;
						if points.contains(&(p.0, y))
							|| self.board.get_cell(p.0, y).is_none_or_ghost()
						{
							continue;
						}
						self.confetti_state.push_tm_point(p.0, p.1, dir);
					}
					Right => {
						let x = p.0 + 1;
						if x >= self.board.cols
							|| points.contains(&(x, p.1))
							|| self.board.get_cell(x, p.1).is_none_or_ghost()
						{
							continue;
						}
						self.confetti_state.push_tm_point(p.0, p.1, dir);
					}
					Bottom => {
						let y = p.1 + 1;
						if y >= self.board.rows
							|| points.contains(&(p.0, y))
							|| self.board.get_cell(p.0, y).is_none_or_ghost()
						{
							continue;
						}
						self.confetti_state.push_tm_point(p.0, p.1, dir);
					}
					Left => {
						if p.0 == 0 {
							continue;
						}
						let x = p.0 - 1;
						if points.contains(&(x, p.1))
							|| self.board.get_cell(x, p.1).is_none_or_ghost()
						{
							continue;
						}
						self.confetti_state.push_tm_point(p.0, p.1, dir);
					}
				}
			}
		}
	}
}
