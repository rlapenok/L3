use axum::async_trait;
use tokio::task::JoinHandle;

#[async_trait]
pub trait MessageQueueSender {
    fn run_sender(&self) -> JoinHandle<()>;
    async fn stop_sender(&self);
}
