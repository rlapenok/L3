use std::error::Error;

use app::create_app;
use axum::serve;
use cli::AppCli;
use infrastructe::app_state::gracefull_shutdown;

mod api;
mod app;
mod cli;
mod config;
mod domain;
mod errors;
mod infrastructe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = AppCli::run();
    let listener = cli.to_listener().await?;
    let app_cfg = cli.to_app_config()?;
    let state = app_cfg.to_state().await?;
    let app = create_app().with_state(state.clone());
    serve(listener, app).with_graceful_shutdown(gracefull_shutdown(state)).await?;
    Ok(())
}
