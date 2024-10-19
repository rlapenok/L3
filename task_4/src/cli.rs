use std::{error::Error, path::PathBuf};

use clap::Parser;
use tokio::net::TcpListener;

use crate::config::AppConfig;
#[derive(Parser)]
pub struct ServerCli {
    #[arg(short = 'a')]
    address: String,
    #[arg(short = 'p')]
    config_path: PathBuf,
}

impl ServerCli {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn to_config(self) -> Result<AppConfig, Box<dyn Error>> {
        AppConfig::load(self.config_path)
    }
    pub async fn to_listener(&self) -> Result<TcpListener, Box<dyn Error>> {
        let listener = TcpListener::bind(&self.address).await?;
        Ok(listener)
    }
}
