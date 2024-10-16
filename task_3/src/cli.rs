use std::error::Error;

use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser)]
pub struct Cli{
    #[arg(short='a')]
    address:String
}
impl Cli{
    pub fn new()->Self{
        Self::parse()
    }
    pub async fn get_listner(&self)->Result<TcpListener,Box<dyn Error>>{
        let listner=TcpListener::bind(&self.address).await?;
        Ok(listner)
    }
}