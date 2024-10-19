use std::error::Error;

use axum::async_trait;
use tokio::task::JoinHandle;

#[async_trait]
pub trait TableChangeNotifier {
    async fn run_notifier(&self) -> JoinHandle<()>;
    async fn stop_notifier(&self) -> Result<(), Box<dyn Error>>;
}
