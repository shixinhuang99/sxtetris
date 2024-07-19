mod app;
mod common;
mod consts;
mod global;
mod handler;
mod save;
mod state;
mod term;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() {
	#[cfg(feature = "_dev")]
	init_log();

	if let Err(err) = run().await {
		eprintln!("{}", err);
	}
}

async fn run() -> Result<()> {
	App::new()?.run().await
}

#[cfg(feature = "_dev")]
fn init_log() {
	use std::fs;

	use simplelog::{Config, LevelFilter, WriteLogger};

	let log_file = fs::File::create("trace.log").unwrap();

	WriteLogger::init(LevelFilter::Trace, Config::default(), log_file).unwrap();
}
