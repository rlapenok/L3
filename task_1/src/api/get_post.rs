use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    domain::{models::Post, social_network_service::SocialNetworkService},
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

pub async fn get_post(
    State(state): State<ServerState>,
    Path(post_uid): Path<Uuid>,
) -> Result<Post, ServerErrors> {
    let post = state.get_post(post_uid).await?;
    Ok(post)
}
