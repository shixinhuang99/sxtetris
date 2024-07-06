pub const START_MENU_ITEMS: [&str; 6] =
	["PLAY", "SCORES", "SETTING", "HELP", "ABOUT", "QUIT"];

pub mod start_menu_idx {
	pub const PLAY: usize = 0;
	pub const SCORES: usize = 1;
	pub const SETTING: usize = 2;
	pub const HELP: usize = 3;
	pub const ABOUT: usize = 4;
	pub const QUIT: usize = 5;
}

pub const PAUSE_MENU_ITEMS: [&str; 6] =
	["RESUME", "NEW GAME", "SCORES", "SETTING", "HELP", "QUIT"];

pub mod pause_menu_idx {
	pub const RESUME: usize = 0;
	pub const NEW_GAME: usize = 1;
	pub const SCORES: usize = 2;
	pub const SETTING: usize = 3;
	pub const HELP: usize = 4;
	pub const QUIT: usize = 5;
}

pub const GAME_OVER_MENU_ITEMS: [&str; 3] = ["NEW GAME", "SCORES", "QUIT"];

pub mod game_over_menu_idx {
	pub const NEW_GAME: usize = 0;
	pub const SCORES: usize = 1;
	pub const QUIT: usize = 2;
}
