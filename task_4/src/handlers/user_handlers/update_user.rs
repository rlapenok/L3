use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    domain::{user_service::UserService, users_models::UpdateUser},
    handlers::json_extractor_with_validation::JsonExtractor,
    server_errors::ServerError,
};

use super::requests::UpdateUserRequest;

pub async fn update_user<S>(
    State(state): State<S>,
    Path(user_id): Path<Uuid>,
    JsonExtractor(req): JsonExtractor<UpdateUserRequest>,
) -> Result<(), ServerError>
where
    S: UserService,
{
    let user = UpdateUser::new(user_id, req.name, req.email);
    state.update_user(user).await?;
    Ok(())
}
