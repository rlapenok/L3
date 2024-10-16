use axum::extract::State;

use crate::{
    domain::models::GetMessage, errors::ServerErrors, infrastructure::server_state::ServerState,
};

use super::{request_structs::JoinLeaveGetMessagesRoom, response_structs::GetMessagesReposne};

pub async fn messages(
    State(state): State<ServerState>,
    get_messages: JoinLeaveGetMessagesRoom,
) -> Result<GetMessagesReposne, ServerErrors> {
    let get_messages = GetMessage::from(get_messages);
    let room_name = get_messages.get_room_name();
    let messages = state.get_messages(get_messages).await?;
    let resp = GetMessagesReposne {
        room_name,
        messages,
    };
    Ok(resp)
}
