use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    domain::social_network_service::SocialNetworkService,
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

pub async fn like_post(
    State(state): State<ServerState>,
    Path(post_uid): Path<Uuid>,
) -> Result<(), ServerErrors> {
    state.like_post(post_uid).await?;
    Ok(())
}
