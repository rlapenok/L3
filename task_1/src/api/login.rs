use axum::extract::State;

use crate::{
    domain::{models::User, social_network_service::SocialNetworkService},
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

use super::{request_structs::LoginRequest, response_structs::LoginResponse};

pub async fn login(
    State(state): State<ServerState>,
    data: LoginRequest,
) -> Result<LoginResponse, ServerErrors> {
    let data = User::from(data);
    let result = state.login(data).await?;
    Ok(LoginResponse(result))
}
