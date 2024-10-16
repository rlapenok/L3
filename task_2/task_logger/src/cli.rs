use std::{error::Error, path::PathBuf};

use clap::Parser;
use tokio::fs::OpenOptions;

use crate::task_logger::logger::TaskLogger;

#[derive(Parser)]
pub struct TaskLoggerCli {
    #[arg(short = 'p', long)]
    path: PathBuf,
}

impl TaskLoggerCli {
    pub async fn to_task_logger(self) -> Result<TaskLogger, Box<dyn Error>> {
        let offset_file = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open("offset.txt")
            .await?;
        TaskLogger::new(self.path, offset_file)
    }
}
