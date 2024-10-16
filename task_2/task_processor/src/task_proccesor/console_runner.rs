use std::path::PathBuf;

use async_trait::async_trait;

use crate::{errors::CommonErrors, models::ProcessedTask};

use super::{job_runner::JobRunner, utils::get_task_from_file};

pub struct ConsoleRunner;

#[async_trait]
impl JobRunner for ConsoleRunner {
    async fn get_gob(&self, path_to_task: PathBuf) -> Result<ProcessedTask, CommonErrors> {
        let created_task = get_task_from_file(path_to_task).await?;
        println!("{:?}", created_task);
        let proccesed_task = ProcessedTask::new(created_task.uuid, created_task.created_at, Ok(()));
        Ok(proccesed_task)
    }
}
