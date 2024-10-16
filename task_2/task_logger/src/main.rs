use std::{env, error::Error};

use clap::Parser;
use cli::TaskLoggerCli;

mod cli;
mod errors;
mod task_logger;


//cargo run -- -p ../tests/logs/ 


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    let cli = TaskLoggerCli::parse();
    let task_logger = cli.to_task_logger().await?;
    task_logger.run().await?;
    Ok(())
}
