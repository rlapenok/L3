use std::error::Error;

use app::create_app;
use axum::serve;
use cli::AppCli;
use infrastructe::app_state::gracefull_shutdown;
use tracing::info;

mod api;
mod app;
mod cli;
mod config;
mod domain;
mod infrastructe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = AppCli::run();
    let listener = cli.to_listener().await?;
    let cfg = cli.to_app_config()?;
    cfg.run_tracing()?;
    let state = cfg.to_state().await?;
    let app = create_app().with_state(state.clone());
    info!(
        "The server is running on: {:?}",
        listener.local_addr().expect("get listener address")
    );
    serve(listener, app).with_graceful_shutdown(gracefull_shutdown(state)).await?;
    Ok(())
}
