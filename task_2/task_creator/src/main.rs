use std::error::Error;

use api::save_task;
use axum::{routing::post, serve, Router};
use cli::TaskCreatorCli;



mod api;
mod cli;
mod request_models;
mod state;
mod errors;
mod domain;
mod infrastracture;

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>>{
    let cli=TaskCreatorCli::new();
    let listner=cli.get_listener().await?;
    let state=cli.to_state().await?;
    let router=Router::new().route("/save_task",post(save_task)).with_state(state);
    println!("Server start on: {}",listner.local_addr().unwrap());
    serve(listner, router).await?;
    Ok(())
}
