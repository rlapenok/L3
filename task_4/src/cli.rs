use std::error::Error;

use clap::Parser;
use tokio::net::TcpListener;
#[derive(Parser)]
pub struct ServerCli{
    #[arg(short='a')]
    address:String
}

impl ServerCli{
    pub fn new()->Self{
        Self::parse()
    }

    pub async fn to_listener(self)->Result<TcpListener,Box<dyn Error>>{
        let listener=TcpListener::bind(self.address).await?;
        Ok(listener)
    }
}