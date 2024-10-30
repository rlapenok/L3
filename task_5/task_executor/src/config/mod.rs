use std::error::Error;

use confique::Config;
use executor_config::ExecutorConfig;
use flume::{unbounded, Receiver, Sender};
use last_task::LastTaskManagerConfig;
use log::{error, info};
use redis::Client;
use redis_config::RedisConfig;
use tracing_config::TracingConfig;

use crate::{
    domain::{file_manager::FileManager, models::Task},
    infrastructe::{
        app::App, executor::Executor, last_task_manager::LastTaskManager, redis_client::RedisClient,
    },
};

mod executor_config;
mod last_task;
mod redis_config;
mod tracing_config;

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    last_task_manager: LastTaskManagerConfig,
    #[config(nested)]
    redis: RedisConfig,
    #[config(nested)]
    executor: ExecutorConfig,
    #[config(nested)]
    tracing: TracingConfig,
}

impl AppConfig {
    async fn ro_redis_receiver(
        &self,
        sender: Sender<Task>,
        last_task_id: String,
    ) -> Result<RedisClient, Box<dyn Error>> {
        let redis_opt = self.redis.to_connect_info();
        let client = Client::open(redis_opt).inspect_err(|err| {
            error!("Error while open redis client: {}", err);
        })?;
        client
            .get_multiplexed_tokio_connection()
            .await
            .inspect_err(|err| {
                error!("Error while ping redis client: {}", err);
            })?;

        Ok(RedisClient::new(client, sender, last_task_id))
    }
    async fn to_last_task_manager(&self) -> Result<FileManager, Box<dyn Error>> {
        let file = self.last_task_manager.get_last_task_file().await?;
        Ok(FileManager::new(file))
    }
    fn to_executor(&self, receiver: Receiver<Task>) -> Executor {
        let url = &self.executor.url;
        let num_workers = self.executor.num_workers;
        Executor::new(num_workers, receiver, url)
    }
    pub fn get_executor_size(&self) -> usize {
        self.executor.num_workers
    }
    pub fn run_tracing(&self) -> Result<(), Box<dyn Error>> {
        self.tracing.run_sub()
    }

    pub async fn to_app(&self) -> Result<App<RedisClient, FileManager, Executor>, Box<dyn Error>> {
        let last_task_manager = self.to_last_task_manager().await?;
        info!("LastTaskManager - OK");
        let last_task_id = last_task_manager.get_last_task_id().await?;
        let (sender, receiver) = unbounded();
        let redis_receiver = self.ro_redis_receiver(sender, last_task_id).await?;
        info!("RedisReceiver - OK");
        let executor = self.to_executor(receiver);
        info!("TaskExecutor (internal:workers) - OK");
        let app = App::new(redis_receiver, last_task_manager, executor);
        Ok(app)
    }
}
