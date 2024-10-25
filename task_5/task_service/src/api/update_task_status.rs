use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{domain::task_service::TaskService, errors::ServerError};

pub async fn update_task_status<T>(
    State(state): State<T>,
    Path(task_id): Path<Uuid>,
) -> Result<(), ServerError>
where
    T: TaskService,
{
    state.complete_task(task_id).await?;
    Ok(())
}
