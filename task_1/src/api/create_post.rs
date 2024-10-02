use axum::{extract::State, Extension};
use uuid::Uuid;

use crate::{
    domain::{models::Post, social_network_service::SocialNetworkService},
    errors::ServerErrors,
    infrastructe::server_state::ServerState,
};

use super::{request_structs::CreatePostRequest, response_structs::CreatePostResponse};

#[axum::debug_handler]
pub async fn create_post(
    State(state): State<ServerState>,
    Extension(user_uid):Extension<Uuid>,
    data:CreatePostRequest
) -> Result<CreatePostResponse, ServerErrors> {
    let data = Post::new(user_uid, data.msg);
    let post_uid=data.post_uid;
    state.create_post(data).await?;
    Ok(CreatePostResponse { post_uid })
}
