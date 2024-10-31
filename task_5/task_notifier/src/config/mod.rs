mod file_manager_config;
mod redis_config;
mod tracing_config;

use std::error::Error;

use confique::Config;
use file_manager_config::LastNotifierConfig;
use log::info;
use redis_config::RedisConfig;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_util::sync::CancellationToken;
use tracing_config::TracingConfig;

use crate::{
    domain::{last_notifier_tasks::LastNotifierTasks, models::{LastTasks, Task}, redis_notifier::RedisNotifier},
    infrastructe::{app_state::AppState, file_manager::FileManager, redis_stream::RedisStream},
};

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    redis: RedisConfig,
    #[config(nested)]
    tracing: TracingConfig,
    #[config(nested)]
    last_notifier_tasks: LastNotifierConfig,
}

impl AppConfig {
    async fn to_redis_notifier(
        &self,
        cancellation_token: CancellationToken,
        completed_sender: UnboundedSender<Task>,
        non_completed_sender: UnboundedSender<Task>,
        last_tasks:LastTasks
    ) -> Result<RedisStream, Box<dyn Error>> {
        let pool = self.redis.create_pool().await?;
        Ok(RedisStream::new(
            pool,
            cancellation_token,
            completed_sender,
            non_completed_sender,
            last_tasks
        ))
    }
    async fn to_last_notifier_tasks(&self) -> Result<FileManager, Box<dyn Error>> {
        let file = self.last_notifier_tasks.get_last_task_file().await?;
        Ok(FileManager::new(file))
    }
    pub async fn to_state(&self) -> Result<AppState<RedisStream, FileManager>, Box<dyn Error>> {
        let lasts_notifier_tasks = self.to_last_notifier_tasks().await?;
        let last_tasks=lasts_notifier_tasks.get_last_tasks().await?;
        let cancellation_token = CancellationToken::new();
        let redis_notifier_token = cancellation_token.child_token();
        let (completed_sender, completed_receiver) = unbounded_channel();
        let (non_completed_sender, non_completed_receiver) = unbounded_channel();
        let redis_notifier = self
            .to_redis_notifier(redis_notifier_token, completed_sender, non_completed_sender,last_tasks)
            .await?;
        info!("RedisNotifier - OK");
        info!("LastNotifierTasks - OK");
        redis_notifier.run().await?;
        Ok(AppState::new(
            redis_notifier,
            lasts_notifier_tasks,
            non_completed_receiver,
            completed_receiver,
            cancellation_token,
        ))
    }
    pub fn run_tracing(&self) -> Result<(), Box<dyn Error>> {
        self.tracing.run_sub()?;
        Ok(())
    }
}
