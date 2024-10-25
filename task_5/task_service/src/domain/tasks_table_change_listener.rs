use std::error::Error;

use axum::async_trait;
use tokio::task::JoinHandle;
#[async_trait]
pub trait TasksTableChangesListener {
    fn run_listener(&self) -> JoinHandle<()>;
    async fn stop_listener(&self) -> Result<(), Box<dyn Error>>;
}