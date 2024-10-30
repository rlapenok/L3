use std::{error::Error, path::PathBuf};

use clap::Parser;
use confique::Config;
use log::error;

use crate::config::AppConfig;
#[derive(Parser)]
pub struct AppCli {
    #[arg(short = 'p')]
    path: PathBuf,
}
impl AppCli {
    pub fn run() -> Self {
        Self::parse()
    }
    pub fn to_config(&self) -> Result<AppConfig, Box<dyn Error>> {
        let cfg = AppConfig::from_file(&self.path).inspect_err(|err| {
            error!("Error while parse config file: {}", err);
        })?;
        Ok(cfg)
    }
}
