use axum::extract::State;

use crate::{
    domain::{user_service::UserService, users_models::User},
    handlers::{
        json_extractor_with_validation::JsonExtractor, reponses::CreateUserProductResponse,
    },
    server_errors::ServerError,
};

use super::requests::CreateUserRequest;

pub async fn create_user<S>(
    State(state): State<S>,
    JsonExtractor(req): JsonExtractor<CreateUserRequest>,
) -> Result<CreateUserProductResponse, ServerError>
where
    S: UserService,
{
    let user = User::from(req);
    let user_id = user.id;
    state.create_user(user).await?;
    Ok(CreateUserProductResponse { id: user_id })
}
