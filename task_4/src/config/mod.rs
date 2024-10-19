use std::{error::Error, path::PathBuf};

use confique::Config;
use postgres_config::PostgresConfig;
use sqlx::PgPool;
use tokio::sync::mpsc::unbounded_channel;
use tokio_util::sync::CancellationToken;

use crate::{
    domain::{change_notifier::TableChangeNotifier, utils::ToChangeNotifier},
    infrastructe::{notifier::Notifier, postgres_repo::PostgresRepo, server_state::ServerState},
};

mod postgres_config;
mod utils;

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    postgres: PostgresConfig,
}

impl AppConfig {
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let config = Self::from_file(path)?;
        Ok(config)
    }

    async fn to_postgres_pool(&self) -> Result<PgPool, sqlx::Error> {
        let conn_opt = self.postgres.to_connect_opt();
        let pool_opt = self.postgres.to_pool_opt();
        pool_opt.connect_with(conn_opt).await
    }
    async fn to_postgres_repo(&self) -> Result<PostgresRepo, Box<dyn Error>> {
        let pool = self.to_postgres_pool().await?;
        let repo = PostgresRepo(pool);
        Ok(repo)
    }
    pub async fn to_state(&self) -> Result<ServerState<PostgresRepo, Notifier>, Box<dyn Error>> {
        let repo = self.to_postgres_repo().await?;
        //create cancellation_token for notifier
        //TODO and for kafka producer
        let cancellation_token = CancellationToken::new();
        let cancellation_token_for_notifier = cancellation_token.child_token();
        //create channel for tables change notifier and kafka producer
        let (sender, _) = unbounded_channel();
        //create change notifier
        let notifier = repo
            .to_change_notifier(cancellation_token_for_notifier, sender)
            .await?;
        notifier.run_notifier().await;
        let state = ServerState::new(repo, notifier, cancellation_token);
        Ok(state)
    }
}
