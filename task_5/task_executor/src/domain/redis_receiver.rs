use async_trait::async_trait;
use redis::RedisResult;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait RedisReceiver {
    async fn run(&self, token: CancellationToken) -> RedisResult<JoinHandle<()>>;
    async fn stop(self) -> String;
}
