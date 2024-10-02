use std::{error::Error, sync::Arc};

use bb8::Pool as Bb8Pool;
use bb8_postgres::PostgresConnectionManager;
use clap::Parser;
use cli::Cli;
use confique::Config;
use migration_config::MigartionConfig;
use postgres_config::PostgresConfig;
use server_config::ServerConfig;
use tokio::net::TcpListener;
use tokio_postgres::Client;
use utils::get_migrations;

use crate::{
    domain::{social_network_repository::SocialNetworkRepository, token_manager::TokenManager},
    infrastructe::{repository::Repository, secret_vault::SecretVault},
};

pub mod cli;
pub mod migration_config;
pub mod postgres_config;
pub mod server_config;
pub mod utils;

#[derive(Config, Debug)]
pub struct AppConfig {
    #[config(nested)]
    server_config: ServerConfig,
    #[config(nested)]
    postgres_config: PostgresConfig,
    #[config(nested)]
    migration_config: MigartionConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let cli = Cli::parse();
        let app_config = AppConfig::from_file(cli.path)?;
        Ok(app_config)
    }

    async fn create_client(&self) -> Result<Client, Box<dyn Error>> {
        let config = self.postgres_config.get_connection_config();
        let (client, connection) = config.0.connect(config.1).await?;
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("Connetion error to postgres: {}", err)
            }
        });
        Ok(client)
    }
    pub async fn get_listner(&self) -> Result<TcpListener, Box<dyn Error>> {
        let listner_addr = self.server_config.get_listner_addr();
        let listner = TcpListener::bind(listner_addr).await?;
        Ok(listner)
    }
    pub async fn run_migration(&self) -> Result<(), Box<dyn Error>> {
        let client = self.create_client().await?;
        let migrations = get_migrations(&self.migration_config.path)?;
        client
            .batch_execute(&migrations)
            .await
            .inspect_err(|err| eprintln!("Error while run migrations: {}", err))?;
        println!("All migration succsess");
        Ok(())
    }
    pub async fn create_repository(
        &self,
    ) -> Result<Arc<dyn SocialNetworkRepository + Send + Sync>, Box<dyn Error>> {
        let config = self.postgres_config.get_connection_config();
        let manager = PostgresConnectionManager::new(config.0, config.1);
        let pool = Bb8Pool::builder()
            .build(manager)
            .await
            .inspect_err(|err| eprintln!("Error while crate postgres pool: {}", err))?;
        let repo = Repository::new(pool);
        Ok(Arc::new(repo))
    }
    pub fn create_token_manager(self) -> Arc<dyn TokenManager + Send + Sync> {
        let info = self.server_config.get_info_to_token_manager();
        let token_manager = SecretVault::new(info.0, info.1);
        Arc::new(token_manager)
    }
}
