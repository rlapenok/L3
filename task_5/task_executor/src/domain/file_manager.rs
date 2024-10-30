use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use log::error;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use crate::infrastructe::last_task_manager::LastTaskManager;

pub struct FileManager(Arc<Mutex<File>>);

impl FileManager {
    pub fn new(file: File) -> Self {
        Self(Arc::new(Mutex::new(file)))
    }
}

#[async_trait]
impl LastTaskManager for FileManager {
    async fn get_last_task_id(&self) -> Result<String, Box<dyn Error>> {
        let mut guard = self.0.lock().await;
        let mut buff = String::new();
        guard
            .read_to_string(&mut buff)
            .await
            .inspect_err(|err| error!("LastTaskManager ---- error while read file: {}", err))?;
        Ok(buff.trim().to_owned())
    }
    async fn save_last_task_id(&self, task: String) -> Result<(), Box<dyn Error>> {
        let mut guard = self.0.lock().await;
        guard.set_len(0).await.inspect_err(|err| {
            error!("LastTaskManager ---- error while set len = 0 : {}", err);
        })?;
        guard.write_all(task.as_bytes()).await.inspect_err(|err| {
            error!(
                "LastTaskManager ---- error while save last task_id: {}",
                err
            );
        })?;
        Ok(())
    }
}
