mod postgres_config;
use std::error::Error;

use confique::Config;
use postgres_config::PostgresConfig;

use crate::infrastructe::{app_state::AppState, postgres_repo::PostgresRepo};

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    postgres: PostgresConfig,
}

impl AppConfig {
    async fn to_task_repository(&self) -> Result<PostgresRepo, Box<dyn Error>> {
        let pool = self.postgres.to_pool().await?;
        let repo = PostgresRepo::new(pool);
        Ok(repo)
    }

    pub async fn to_state(&self) -> Result<AppState<PostgresRepo>, Box<dyn Error>> {
        let repo = self.to_task_repository().await?;
        let state = AppState::new(repo);
        Ok(state)
    }
}
