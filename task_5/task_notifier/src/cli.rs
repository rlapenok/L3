use std::{error::Error, path::PathBuf};

use clap::Parser;
use confique::Config;

use tokio::net::TcpListener;
use tracing::error;

use crate::config::AppConfig;

#[derive(Parser)]
pub struct AppCli {
    #[arg(short = 'p')]
    path: PathBuf,
    #[arg(short = 'a')]
    address: String,
}
impl AppCli {
    pub fn run() -> Self {
        Self::parse()
    }
}

impl AppCli {
    pub async fn to_listener(&self) -> Result<TcpListener, Box<dyn Error>> {
        let listener = TcpListener::bind(&self.address).await.inspect_err(|err| {
            error!("Error while create TcpListener:{}", err);
        })?;
        Ok(listener)
    }
    pub fn to_app_config(&self) -> Result<AppConfig, Box<dyn Error>> {
        let cfg = AppConfig::from_file(&self.path).inspect_err(|err| {
            error!(
                "Error while parse file: {:?} for load config: {}",
                &self.path, err
            );
        })?;
        Ok(cfg)
    }
}
