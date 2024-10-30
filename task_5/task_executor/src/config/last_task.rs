use std::{error::Error, path::PathBuf};

use confique::Config;
use log::error;
use tokio::fs::{File, OpenOptions};

#[derive(Config)]
pub(crate) struct LastTaskManagerConfig {
    path: PathBuf,
}

impl LastTaskManagerConfig {
    pub async fn get_last_task_file(&self) -> Result<File, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.path)
            .await
            .inspect_err(|err| {
                error!("Error while open/create file for LastTaskManager: {}", err)
            })?;
        Ok(file)
    }
}
