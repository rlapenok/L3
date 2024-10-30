

use axum::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use super::{
    models::{NewTask, TableChange, Task},
   table_change_listener::TableChangesListener
};

#[async_trait]
pub trait TaskRepository {
    type ChangeListener: TableChangesListener;
    async fn create_task(&self, task: NewTask) -> Result<(), sqlx::Error>;
    async fn get_task(&self, task_id: Uuid) -> Result<Task, sqlx::Error>;
    async fn complete_task(&self, task_id: Uuid,time:DateTime<Utc>) -> Result<(), sqlx::Error>;
    async fn to_change_listener(
        &self,
        cancellation_token: CancellationToken,
        sender: UnboundedSender<TableChange>,
    ) -> Result<Self::ChangeListener, sqlx::Error>;
    async fn close(&self);
}
