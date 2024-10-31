use std::{error::Error, sync::Arc};

use axum::async_trait;
use log::error;
use serde_json::error::Category;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use crate::domain::{last_notifier_tasks::LastNotifierTasks, models::LastTasks};

#[derive(Clone)]
pub struct FileManager(Arc<Mutex<File>>);

impl FileManager {
    pub fn new(file: File) -> Self {
        Self(Arc::new(Mutex::new(file)))
    }
}

#[async_trait]
impl LastNotifierTasks for FileManager {
    async fn get_last_tasks(&self) -> Result<LastTasks, Box<dyn Error>> {
        let mut guard = self.0.lock().await;
        let mut buff = String::new();
        guard
            .read_to_string(&mut buff)
            .await
            .inspect_err(|err| error!("LastNotifierTasks  ---- error while read file: {}", err))?;
        let tasks;
        match serde_json::from_str::<LastTasks>(buff.trim()) {
            Ok(last_tasks) => tasks=Ok(last_tasks),
            Err(err) => {
                if err.classify() == Category::Eof {
                    tasks=Ok(LastTasks::default())
                }else {
                    tasks=Err(err)
                }
                
            }
        };
        let tasks=tasks?;
        Ok(tasks)
    }
    async fn save_last_tasks(&self, tasks: LastTasks) -> Result<(), Box<dyn Error>> {
        let mut guard = self.0.lock().await;
        guard.set_len(0).await.inspect_err(|err| {
            error!("LastNotifierTasks ---- error while set len = 0 : {}", err);
        })?;
        let tasks = serde_json::to_string_pretty(&tasks)?;
        guard.write_all(tasks.as_bytes()).await.inspect_err(|err| {
            error!(
                "LastNotifierTasks ---- error while save last task_id: {}",
                err
            );
        })?;
        Ok(())
    }
}
