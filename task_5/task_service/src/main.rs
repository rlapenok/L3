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
mod errors;
mod infrastructe;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //run cli
    let cli = AppCli::run();
    //cretae TcpListener
    let listener = cli.to_listener().await?;
    //convert cli to config
    let app_cfg = cli.to_app_config()?;

    app_cfg.run_tracing_subscriber()?;
    //convert config to state
    let state = app_cfg.to_state().await?;
    //create app
    let app = create_app().with_state(state.clone());
    //run server
    info!(
        "The server is running on: {:?}",
        listener.local_addr().unwrap()
    );
    serve(listener, app)
        .with_graceful_shutdown(gracefull_shutdown(state))
        .await?;
    Ok(())
}