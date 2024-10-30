mod postgres_config;
mod redis_config;
mod tracing_config;
use std::{error::Error, path::Path};

use confique::Config;
use postgres_config::PostgresConfig;
use redis::Client;
use redis_config::RedisConfig;
use sqlx::migrate::Migrator;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_config::TracingConfig;

use crate::{
    domain::{models::TableChange, task_repository::TaskRepository},
    infrastructe::{
        app_state::AppState, postgres_repo::PostgresRepo, redis_client::RedisClient,
        table_listener::TableListener,
    },
};

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    postgres: PostgresConfig,
    #[config(nested)]
    redis: RedisConfig,
    #[config(nested)]
    tracing: TracingConfig,
}

impl AppConfig {
    async fn to_task_repository(&self) -> Result<PostgresRepo, Box<dyn Error>> {
        let pool = self.postgres.to_pool().await?;
        let path_migrations = self.postgres.get_path_migrations();
        let migrator = Migrator::new(Path::new(path_migrations))
            .await
            .inspect_err(|err| {
                error!("Error while create migartor for run migrations: {}", err);
            })?;
        migrator.run(&pool).await.inspect_err(|err| {
            error!("Error while run migrations: {}", err);
        })?;
        info!("Migrations applied successfully");
        let repo = PostgresRepo::new(pool);
        Ok(repo)
    }
    async fn to_message_queue_sender(
        &self,
        cancellation_token: CancellationToken,
        recv: UnboundedReceiver<TableChange>,
    ) -> Result<RedisClient, Box<dyn Error>> {
        let redis_opt = self.redis.to_connect_info();
        let client = Client::open(redis_opt)
            .inspect_err(|err| error!("Error while open client for Redis: {}", err))?;
        client
            .get_multiplexed_tokio_connection()
            .await
            .inspect_err(|err| error!("Error ping Redis: {}", err))?;

        Ok(RedisClient::new(client, cancellation_token, recv))
    }

    pub async fn to_state(
        &self,
    ) -> Result<AppState<PostgresRepo, TableListener, RedisClient>, Box<dyn Error>> {
        let cancellation_token = CancellationToken::new();
        let token_for_listener = cancellation_token.child_token();
        let token_for_message_queue_sender = cancellation_token.child_token();
        let repo = self.to_task_repository().await?;
        info!("TaskRepository - OK");
        let (sender, receiver) = unbounded_channel();
        let message_queue_sender = self
            .to_message_queue_sender(token_for_message_queue_sender, receiver)
            .await?;
        info!("MessageQueueSender - OK");
        let listener = repo.to_change_listener(token_for_listener, sender).await?;
        info!("TableChangesListener - Ok");
        let state = AppState::new(repo, listener, message_queue_sender, cancellation_token);
        Ok(state)
    }
    pub fn run_tracing_subscriber(&self) -> Result<(), Box<dyn Error>> {
       self.tracing.run_sub()?;
        Ok(())
    }

}

