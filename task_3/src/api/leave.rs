use axum::extract::State;

use crate::{errors::ServerErrors, infrastructure::server_state::ServerState};

use super::request_structs::JoinLeaveGetMessagesRoom;

pub async fn leave(
    State(state): State<ServerState>,
    join_room: JoinLeaveGetMessagesRoom,
) -> Result<(), ServerErrors> {
    state.leave_room(join_room.user_name, join_room.room_name)?;
    Ok(())
}
