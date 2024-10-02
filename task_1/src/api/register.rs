use axum::extract::State;

use crate::{
    domain::{models::User, social_network_service::SocialNetworkService},
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

use super::{request_structs::RegisterRequest, response_structs::RegisterResponse};

pub async fn register(
    State(state): State<ServerState>,
    data: RegisterRequest,
) -> Result<RegisterResponse, ServerErrors> {
    let data = User::from(data);
    let result = state.register(data).await?;
    Ok(RegisterResponse(result.0, result.1))
}
