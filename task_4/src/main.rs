use std::{env, error::Error};

use app::create_app;
use axum::serve;
use cli::ServerCli;
use infrastructe::server_state::gracefull_shutdown;

mod app;
mod cli;
mod config;
mod domain;
mod handlers;
mod infrastructe;
mod server_errors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "debug,sqlx=error");
    env_logger::init();
    let cli = ServerCli::new();
    let listener = cli.to_listener().await?;
    let config = cli.to_config()?;
    let state = config.to_state().await?;
    let state_for_stop = state.clone();
    let app = create_app().with_state(state);
    serve(listener, app)
        .with_graceful_shutdown(gracefull_shutdown(state_for_stop))
        .await?;
    Ok(())
}
