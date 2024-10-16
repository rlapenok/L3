use axum::async_trait;

use crate::errors::ServerError;

use super::models::Task;

#[async_trait]
pub trait TaskSaver:Send+Sync {
    async fn save_task(&self,task:Task)->Result<(),ServerError>;
}