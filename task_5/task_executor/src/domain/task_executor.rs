use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use super::models::Task;

pub type Job = Box<
    dyn Fn(Task) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

#[async_trait]
pub trait TaskExecutor {
    fn run(&self, cancellation_token: CancellationToken);
    fn do_job(&self) -> Job;
    async fn stop(self);
}
