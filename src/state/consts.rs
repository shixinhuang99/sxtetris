pub const START_MENU_ITEMS: [&str; 5] =
	["PLAY", "SCORES", "HELP", "ABOUT", "QUIT"];

pub mod start_menu_idx {
	pub const PLAY: usize = 0;
	pub const SCORES: usize = 1;
	pub const HELP: usize = 2;
	pub const ABOUT: usize = 3;
	pub const QUIT: usize = 4;
}

pub const PAUSE_MENU_ITEMS: [&str; 5] =
	["RESUME", "NEW GAME", "SCORES", "HELP", "QUIT"];

pub mod pause_menu_idx {
	pub const RESUME: usize = 0;
	pub const NEW_GAME: usize = 1;
	pub const SCORES: usize = 2;
	pub const HELP: usize = 3;
	pub const QUIT: usize = 4;
}

pub const GAME_OVER_MENU_ITEMS: [&str; 3] = ["NEW GAME", "SCORES", "QUIT"];

pub mod game_over_menu_idx {
	pub const NEW_GAME: usize = 0;
	pub const SCORES: usize = 1;
	pub const QUIT: usize = 2;
}
