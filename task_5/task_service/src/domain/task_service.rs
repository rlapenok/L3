use axum::async_trait;
use uuid::Uuid;

use super::{
    models::{NewTask, Task},
    service_error::TaskServiceError,
};

#[async_trait]
pub trait TaskService {
    async fn create_task(&self, task: NewTask) -> Result<(), TaskServiceError>;
    async fn get_task(&self, task_id: Uuid) -> Result<Task, TaskServiceError>;
    async fn complete_task(&self, task_id: Uuid) -> Result<(), TaskServiceError>;
}
