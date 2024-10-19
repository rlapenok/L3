use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{domain::user_service::UserService, server_errors::ServerError};

pub async fn delete_user<S>(
    State(state): State<S>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ServerError>
where
    S: UserService,
{
    state.delete_user(user_id).await?;
    Ok(())
}
