use std::{env, error::Error};

use app::{create_app, shutdown};
use axum::serve;
use cli::Cli;
use infrastructure::server_state::ServerState;
use log::info;

mod api;
mod app;
mod domain;
mod errors;
mod infrastructure;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cli=Cli::new();
    let listener=cli.get_listner().await?;
    let state = ServerState::new();
    let shutdown_state = state.clone();
    let app = create_app().with_state(state);
    info!("Start server on:  {}", listener.local_addr()?);
    serve(listener, app)
        .with_graceful_shutdown(shutdown(shutdown_state))
        .await?;

    Ok(())
}
