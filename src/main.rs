mod app;
mod channel;
mod consts;
mod handler;
mod state;
mod term;
mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() {
	if let Err(err) = run().await {
		eprintln!("{}", err);
	}
}

async fn run() -> Result<()> {
	let mut app = App::new()?;

	app.run().await?;

	Ok(())
}
