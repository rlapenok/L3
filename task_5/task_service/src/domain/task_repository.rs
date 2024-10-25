use axum::async_trait;
use uuid::Uuid;

use super::{models::{NewTask, Task}, tasks_table_change_listener::TasksTableChangesListener};

#[async_trait]
pub trait TaskRepository {
    type ChangeListener:TasksTableChangesListener;
    async fn create_task(&self, task: NewTask) -> Result<(), sqlx::Error>;
    async fn get_task(&self, task_id: Uuid) -> Result<Task, sqlx::Error>;
    async fn complete_task(&self, task_id: Uuid) -> Result<(), sqlx::Error>;
    async fn to_change_listener(&self)->Self::ChangeListener;
    async fn close(&self);
}
