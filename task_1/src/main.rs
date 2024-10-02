use std::error::Error;

use app::create_app;
use axum::serve;
use config::AppConfig;
use infrastructe::server_state::ServerState;

mod api;
mod app;
mod config;
mod domain;
mod errors;
mod infrastructe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //load config from file
    let config = AppConfig::load()?;
    //run migrations
    config.run_migration().await?;
    //create listner form config
    let listner=config.get_listner().await?;
    //create_repository
    let repo = config.create_repository().await?;
    //create token_manager for jwt tokens
    let token_manager=config.create_token_manager();
    //create state
    let server_state = ServerState::new(repo,token_manager);
    //create routing
    let app=create_app(server_state);
    //start server
    serve(listner, app).await?;
    Ok(())
}
