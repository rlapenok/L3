use std::{path::PathBuf, sync::Arc};

use axum::async_trait;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{domain::{models::Task, task_saver::TaskSaver}, errors::ServerError};

#[derive(Clone)]
pub struct Saver{
    dir:Arc<PathBuf>
}

impl Saver{
    pub fn new(dir:Arc<PathBuf>)->Self{
        Self { dir }
    }
}

#[async_trait]
impl TaskSaver for Saver{
    async fn save_task(&self,task:Task)->Result<(),ServerError>{
        let dir=self.dir.clone();
        let file_name=format!("{}.json",task.uuid);
        let task=serde_json::to_string_pretty(&task)?;
        let path=dir.join(file_name);
        let mut file=OpenOptions::new().create(true).append(true).open(path).await?;
        file.write_all(task.as_bytes()).await?;
        Ok(())
    }
}