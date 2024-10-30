

use axum::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgListener, ConnectOptions, PgPool};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use tracing::{error, info,instrument,span::Span};
use uuid::Uuid;

use crate::domain::{
    models::{NewTask, TableChange, Task},
    task_repository::TaskRepository,
};

use super::{table_listener::TableListener, utils::{get_span_id, get_trace_id}};
#[derive(Clone)]

pub struct PostgresRepo(PgPool);
impl PostgresRepo {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl TaskRepository for PostgresRepo {
    type ChangeListener = TableListener;
    #[instrument(skip_all, name = "TaskRepository::create_task",fields(%task_id=task.id))]
    async fn create_task(&self, task: NewTask) -> Result<(), sqlx::Error> {
        let mut tx = self.0.begin().await.inspect_err(|err| {
            error!("Error while start transaction: {}", err);
        })?;
        //for distributing tracing
        let span=Span::current();
        let trace_id=get_trace_id(&span);
        let span_id=get_span_id(&span);

        sqlx::query("INSERT INTO tasks (id,description,created_at,completed_at,trace_id,span_id) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(task.id)
            .bind(task.description)
            .bind(task.created_at)
            .bind(task.completed_at)
            .bind(trace_id)
            .bind(span_id)
            .execute(&mut *tx)
            .await
            .inspect_err(|err| error!("Error while execute query :{}", err))?;
        tx.commit()
            .await
            .inspect_err(|err| error!("Error while commit transaction: {}", err))?;
        Ok(())
    }
    #[instrument(skip_all, name = "TaskRepository::get_task",fields(%task_id=task_id))]
    async fn get_task(&self, task_id: Uuid) -> Result<Task, sqlx::Error> {
        let mut tx = self.0.begin().await.inspect_err(|err| {
            error!("Error while start transaction: {}", err);
        })?;
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(task_id)
            .fetch_one(&mut *tx)
            .await
            .inspect_err(|err| error!("Error while execute query :{}", err))?;
        tx.commit()
            .await
            .inspect_err(|err| error!("Error while commit transaction: {}", err))?;
        Ok(task)
    }
    #[instrument(skip_all, name = "TaskRepository::complete_task",fields(%task_id=task_id))]
    async fn complete_task(&self, task_id: Uuid,time:DateTime<Utc>) -> Result<(), sqlx::Error> {
        let mut tx = self.0.begin().await.inspect_err(|err| {
            error!("Error while start transaction: {}", err);
        })?;
        sqlx::query("UPDATE tasks SET completed = TRUE, completed_at = $1 WHERE id = $2 AND completed = FALSE")
            .bind(time)
            .bind(task_id)
            .execute(&mut *tx)
            .await
            .map_or_else(
                Err,
                |result| {
                    if result.rows_affected() > 0 {
                        Ok(())
                    } else {
                        Err(sqlx::Error::RowNotFound)
                    }
                },
            )
            .inspect_err(|err| {
                error!("Error while execute query: {}", err);
            })?;
        tx.commit()
            .await
            .inspect_err(|err| error!("Error while commit transaction: {}", err))?;
        Ok(())
    }

    async fn to_change_listener(
        &self,
        cancellation_token: CancellationToken,
        sender: UnboundedSender<TableChange>,
    ) -> Result<Self::ChangeListener, sqlx::Error> {
        let conn_str = self.0.connect_options().to_url_lossy().to_string();
        let mut listener = PgListener::connect(&conn_str)
            .await
            .inspect_err(|err| error!("Error while create PgListener: {}", err))?;
        listener.listen("inserts").await.inspect_err(|err| {
            error!(
                "Error while add listen channel = @inserts@ to PgListener:{}",
                err
            )
        })?;
        listener.listen("updates").await.inspect_err(|err| {
            error!(
                "Error while add listen channel = @ updates @ to PgListener:{}",
                err
            )
        })?;
        Ok(Self::ChangeListener::new(
            listener,
            cancellation_token,
            sender,
        ))
    }
    #[instrument(skip_all, name = "TaskRepository::close")]
    async fn close(&self) {
        self.0.close().await;
        info!("TaskRepository - STOP")
    }
}
