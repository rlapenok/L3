use axum::extract::{Path, State};
use tracing::{info, instrument, Span};
use uuid::Uuid;

use crate::{domain::task_service::TaskService, errors::ServerError};

use super::response::GetTask;

#[instrument(skip_all,fields(task_id),name="get_task_info_handler")]
pub async fn get_task_info<T>(
    State(state): State<T>,
    Path(task_id): Path<Uuid>,
) -> Result<GetTask, ServerError>
where
    T: TaskService,
{
    let task = state.get_task(task_id).await?;
    Span::current().record("task_id", task_id.to_string());
    let reposne = GetTask::from(task);
    info!("Successfully");
    Ok(reposne)
}
