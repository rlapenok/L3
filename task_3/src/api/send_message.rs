use axum::extract::State;

use crate::{
    domain::models::Message, errors::ServerErrors, infrastructure::server_state::ServerState,
};

use super::request_structs::SendMessage;

pub async fn send_message(
    State(state): State<ServerState>,
    message_request: SendMessage,
) -> Result<(), ServerErrors> {
    let message = Box::new(Message::from(message_request));
    state.send_message(message)?;
    Ok(())
}
