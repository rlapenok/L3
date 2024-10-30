use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait LastTaskManager {
    async fn get_last_task_id(&self) -> Result<String, Box<dyn Error>>;
    async fn save_last_task_id(&self, task: String) -> Result<(), Box<dyn Error>>;
}
