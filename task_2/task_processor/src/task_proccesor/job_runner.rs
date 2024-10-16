use std::path::PathBuf;

use async_trait::async_trait;

use crate::{errors::CommonErrors, models::ProcessedTask};

#[async_trait]
pub trait JobRunner {
    async fn get_gob(&self, path_to_task: PathBuf) -> Result<ProcessedTask, CommonErrors>;
}
