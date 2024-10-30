use std::sync::Arc;

use axum::async_trait;
use chrono::{DateTime, Utc};
use tokio::signal::ctrl_c;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{info,  instrument};
use uuid::Uuid;

use crate::domain::{
    message_queue_sender::MessageQueueSender,
    models::{NewTask, Task},
    service_error::TaskServiceError,
    table_change_listener::TableChangesListener,
    task_repository::TaskRepository,
    task_service::TaskService,
};

#[derive(Clone)]
pub struct AppState<T, L, M> {
    repo: T,
    listener: L,
    task_tracker: TaskTracker,
    message_queue_sender: M,
    cancellation_token: Arc<CancellationToken>,
}

impl<T, L, M> AppState<T, L, M>
where
    T: TaskRepository + Clone + Send + Sync,
    L: TableChangesListener + Clone + Send + Sync,
    M: MessageQueueSender + Clone + Send + Sync,
{
    pub fn new(
        repo: T,
        listener: L,
        message_queue_sender: M,
        cancellation_token: CancellationToken,
    ) -> Self {
        let listener_handler = listener
            .run_listener();
        let message_queue_handler = message_queue_sender.run_sender();
        let task_tracker = TaskTracker::new();
        task_tracker.spawn(listener_handler);
        task_tracker.spawn(message_queue_handler);
        Self {
            repo,
            listener,
            task_tracker,
            message_queue_sender,
            cancellation_token: Arc::new(cancellation_token),
        }
    }
}

#[async_trait]
impl<T, L, M> TaskService for AppState<T, L, M>
where
    T: TaskRepository + Clone + Send + Sync,
    L: Clone + Send + Sync,
    M: Clone + Send + Sync,
{
    #[instrument(skip_all,name="TaskService::create_task",fields(%task_id=task.id))]
    async fn create_task(&self, task: NewTask) -> Result<(), TaskServiceError> {
        self.repo.create_task(task).await?;
        Ok(())
    }
    #[instrument(skip_all,name="TaskService::get_task",fields(%task_id=task_id))]
    async fn get_task(&self, task_id: Uuid) -> Result<Task, TaskServiceError> {
        let task = self.repo.get_task(task_id).await?;
        Ok(task)
    }
    #[instrument(skip_all,name="TaskService::complete_task",fields(%task_id=task_id))]
    async fn complete_task(&self, task_id: Uuid,time:DateTime<Utc>) -> Result<(), TaskServiceError> {
        self.repo.complete_task(task_id,time).await?;
        Ok(())
    }
}

pub async fn gracefull_shutdown<T, L, M>(state: AppState<T, L, M>)
where
    T: TaskRepository + Clone + Send + Sync,
    L: TableChangesListener + Clone + Send + Sync,
    M: MessageQueueSender + Clone + Send + Sync,
{
    ctrl_c().await.expect("failed to install Ctrl+C handler");
    info!("Start gracefull shutdown server");
    state.task_tracker.close();
    state.cancellation_token.cancel();
    state.task_tracker.wait().await;
    state.repo.close().await;
    state.message_queue_sender.stop_sender().await;
    state.listener.stop_listener().await.unwrap();
    info!("Server gracefully shutdown");
}
