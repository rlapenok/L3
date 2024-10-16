use std::{env, error::Error};

use clap::Parser;
use cli::TaskProccesorCli;
use dotenv::dotenv;

mod cli;
mod errors;
mod models;
mod task_proccesor;


//cargo run -- -p ../tests/tasks/ -l ../tests/logs -n 4 kafka 


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env::set_var("RUST_LOG", "INFO,rdkafka=error");
    env_logger::init();

    let cli = TaskProccesorCli::parse();
    let processor = cli.to_task_processor().await?;
    processor.run().await?;
    Ok(())
}
