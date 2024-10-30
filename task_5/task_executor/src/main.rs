use std::error::Error;

use cli::AppCli;
use log::info;

mod cli;
mod config;
mod domain;
mod infrastructe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = AppCli::run();
    let cfg = cli.to_config()?;
    cfg.run_tracing()?;
    let executor = cfg.to_app().await?;
    info!("TaskExecuter running");
    executor.run().await?;
    Ok(())
}
