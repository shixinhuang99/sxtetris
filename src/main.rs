mod app;
mod audio;
mod common;
mod consts;
mod handler;
mod state;
mod term;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() {
	#[cfg(feature = "_dev")]
	simplelog::WriteLogger::init(
		simplelog::LevelFilter::Trace,
		simplelog::Config::default(),
		std::fs::File::create("trace.log").unwrap(),
	)
	.unwrap();

	if let Err(err) = run().await {
		eprintln!("{}", err);
	}
}

async fn run() -> Result<()> {
	let mut app = App::new()?;

	app.run().await?;

	Ok(())
}
