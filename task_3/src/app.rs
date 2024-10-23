use axum::{routing::post, Router};
use log::info;
use tokio::signal::ctrl_c;

use crate::{
    api::{get_messages::messages, join::join, leave::leave, send_message::send_message},
    infrastructure::server_state::ServerState,
};

pub fn create_app() -> Router<ServerState> {
    Router::new()
        .route("/join", post(join))
        .route("/leave", post(leave))
        .route("/send_message", post(send_message))
        .route("/messages", post(messages))
}

pub async fn shutdown(server: ServerState) {
    ctrl_c().await.expect("failed to install Ctrl+C handler");
    info!("Start gracefull shutdown server");
    server.shutdown().await;
    info!("Server stopped");
}
