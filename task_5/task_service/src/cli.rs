use std::{error::Error, path::PathBuf};

use clap::Parser;
use confique::Config;
use tokio::net::TcpListener;

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
        let listener = TcpListener::bind(&self.address).await?;
        Ok(listener)
    }
    pub fn to_app_config(&self) -> Result<AppConfig, Box<dyn Error>> {
        let cfg = AppConfig::from_file(&self.path)?;
        Ok(cfg)
    }
}
