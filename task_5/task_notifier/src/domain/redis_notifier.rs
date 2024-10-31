use std::error::Error;

use axum::async_trait;

use super::models::LastTasks;

#[async_trait]
pub trait RedisNotifier {
    async fn run(&self) -> Result<(),Box<dyn Error>>;
    async fn stop(&self)->LastTasks;
}
