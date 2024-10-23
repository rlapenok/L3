use std::error::Error;

use axum::async_trait;
use tokio::task::JoinHandle;

pub struct TableChanges {
    pub table_name: String,
    pub payload: String,
}

#[async_trait]
pub trait TableChangeNotifier {
    fn run_notifier(&self) -> JoinHandle<()>;
    async fn stop_notifier(&self) -> Result<(), Box<dyn Error>>;
}
