use std::error::Error;

use axum::async_trait;

use super::models::LastTasks;

#[async_trait]
pub trait LastNotifierTasks {
    async fn get_last_tasks(&self) -> Result<LastTasks, Box<dyn Error>>;
    async fn save_last_tasks(&self, tasks: LastTasks) -> Result<(), Box<dyn Error>>;
}
