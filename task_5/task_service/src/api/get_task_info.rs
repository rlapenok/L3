use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{domain::task_service::TaskService, errors::ServerError};

use super::response::GetTask;

pub async fn get_task_info<T>(
    State(state): State<T>,
    Path(task_id): Path<Uuid>,
) -> Result<GetTask, ServerError>
where
    T: TaskService,
{
    let task = state.get_task(task_id).await?;
    let reposne = GetTask::from(task);
    Ok(reposne)
}
