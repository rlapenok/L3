use std::error::Error;

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
//run migration     sqlx migrate run --database-url postgres://wb_tech:wb_tech@localhost:5432/L3.4

// run app cargo run -- -a localhost:8080 -p config.toml

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
