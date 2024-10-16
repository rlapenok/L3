use axum::extract::State;

use crate::{errors::ServerErrors, infrastructure::server_state::ServerState};

use super::request_structs::JoinLeaveGetMessagesRoom;

pub async fn join(
    State(state): State<ServerState>,
    join_room: JoinLeaveGetMessagesRoom,
) -> Result<(), ServerErrors> {
    state.join_room(join_room.user_name, join_room.room_name)?;
    Ok(())
}
