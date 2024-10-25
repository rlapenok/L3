use axum::async_trait;
use tokio::signal::ctrl_c;
use uuid::Uuid;

use crate::domain::{
    models::{NewTask, Task},
    service_error::TaskServiceError,
    task_repository::TaskRepository,
    task_service::TaskService,
};

#[derive(Clone)]
pub struct AppState<T> {
    repo: T,
}

impl<T> AppState<T> 
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> TaskService for AppState<T>
where
    T: TaskRepository + Clone + Send + Sync,
{
    async fn create_task(&self, task: NewTask) -> Result<(), TaskServiceError> {
        self.repo.create_task(task).await?;
        Ok(())
    }
    async fn get_task(&self, task_id: Uuid) -> Result<Task, TaskServiceError> {
        let task = self.repo.get_task(task_id).await?;
        Ok(task)
    }
    async fn complete_task(&self, task_id: Uuid) -> Result<(), TaskServiceError> {
        self.repo.complete_task(task_id).await?;
        Ok(())
    }
}



pub async fn gracefull_shutdown<T>(state: AppState<T>)
where
    T: TaskRepository + Clone + Send+Sync,
{
    ctrl_c().await.expect("failed to install Ctrl+C handler");
    state.repo.close().await;
}
