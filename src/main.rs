mod config;
mod gateway;
mod worker;

use anyhow::Result;
use config::Config;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

	/// Handle http requests
    let _ = crate::gateway::run();

	/// Resolve error reports
    let _ = crate::worker::run();

	/// Wait for handler and worker to finish
	loop {}
    Ok(())
}
