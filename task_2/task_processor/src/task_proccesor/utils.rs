use std::{error::Error, path::PathBuf, sync::Arc, time::UNIX_EPOCH};

use tokio::{
    fs::{read_dir, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use crate::{
    errors::CommonErrors,
    models::{CreatedTask, ProcessedTask},
};

use super::job_runner::JobRunner;

pub async fn get_task_from_file(path_to_task: PathBuf) -> Result<CreatedTask, CommonErrors> {
    let mut file = OpenOptions::new().read(true).open(path_to_task).await?;
    let mut buff = String::new();
    file.read_to_string(&mut buff).await?;
    let task = serde_json::from_str::<CreatedTask>(&buff)?;
    Ok(task)
}

pub async fn create_job(
    job_runner: Arc<dyn JobRunner + Send + Sync>,
    path_to_task: PathBuf,
) -> Result<ProcessedTask, CommonErrors> {
    job_runner.get_gob(path_to_task).await
}

pub async fn write_offset(dir: Arc<PathBuf>, file: Arc<Mutex<File>>) -> Result<(), Box<dyn Error>> {
    let mut files = read_dir(&*dir).await?;
    let mut offset_time = 0;
    while let Some(file) = files.next_entry().await? {
        if let Ok(meta) = file.metadata().await {
            if let Ok(time) = meta.created() {
                if let Ok(time) = time.duration_since(UNIX_EPOCH) {
                    if offset_time < time.as_nanos() {
                        offset_time = time.as_nanos()
                    }
                }
            }
        }
    }
    let mut guard = file.lock().await;
    guard.write_all(offset_time.to_string().as_bytes()).await?;
    Ok(())
}
