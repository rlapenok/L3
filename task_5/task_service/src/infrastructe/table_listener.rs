use std::sync::Arc;

use axum::async_trait;
use sqlx::postgres::PgListener;
use tokio::{spawn, sync::Mutex, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use crate::domain::tasks_table_change_listener::TasksTableChangesListener;

#[derive(Clone)]
pub struct TableListener{
    listener:Arc<Mutex<PgListener>>,
    cancellation_token:Arc<CancellationToken>
}

impl TableListener{
    pub fn new(listener:PgListener,cancellation_token:CancellationToken)->Self{
        Self { listener: Arc::new(Mutex::new(listener)), cancellation_token: Arc::new(cancellation_token) }
    }
}
#[async_trait]
impl TasksTableChangesListener for TableListener{
    fn run_listener(&self) -> JoinHandle<()> {
        let task=spawn(async {});
        task
    }
     async fn stop_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}