use axum::{extract::{Path, State}, Extension};
use uuid::Uuid;

use crate::{
    domain::{models::DeletePost, social_network_service::SocialNetworkService},
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

pub async fn delete_post(
    State(state): State<ServerState>,
    Extension(user_uid):Extension<Uuid>,
    Path(post_uid): Path<Uuid>,
) -> Result<(), ServerErrors> {
    let post=DeletePost{
        user_uid,
        post_uid
    };
    state.delete_post(post).await?;
    Ok(())
}
