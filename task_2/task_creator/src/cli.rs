
use tokio::{fs::create_dir_all, net::TcpListener};
use std::{error::Error, io, path::PathBuf, sync::Arc};
use clap::Parser;

use crate::{errors::CliError, infrastracture::saver::Saver, state::ServerState};



#[derive(Parser)]
pub struct TaskCreatorCli {
    #[arg(short = 'p', long)]
    path: PathBuf,
    #[arg(short = 'a', long)]
    addr:String,
}

impl TaskCreatorCli {
    pub fn new() -> TaskCreatorCli {
        Self::parse()
    }
    pub async fn to_state(self)->Result<ServerState,Box<dyn Error>>
    {
        if self.path.is_file(){
            return Err(Box::new(CliError::NotDirectory(self.path.file_name().unwrap().to_string_lossy().to_string())))
        }
        create_dir_all(&self.path).await?;
                let dir=Arc::new(self.path);
                let directory_saver=Arc::new(Saver::new(dir));
                Ok(ServerState::new(directory_saver))
    }

    pub async fn get_listener(&self)->io::Result<TcpListener>{
        TcpListener::bind(&self.addr).await
    }
}
