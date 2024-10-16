use std::error::Error;

use app::create_app;
use axum::serve;
use cli::ServerCli;

mod cli;
mod handlers;
mod server_errors;
mod app;


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>> {

    let cli=ServerCli::new();
    let listener=cli.to_listener().await?;
     let app=create_app();
     serve(listener, app).await?;   
    Ok(())
}
