use axum::extract::{Path, State};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{domain::task_service::TaskService, errors::ServerError};

use super::requests::{JsonExtractor, UpdateTask};


#[instrument(name = "update_task_handler", skip_all,fields(%task_id=task_id))]
pub async fn update_task_status<T>(
    State(state): State<T>,
    Path(task_id): Path<Uuid>,
    JsonExtractor(req):JsonExtractor<UpdateTask>
) -> Result<(), ServerError>
where
    T: TaskService,
{
    let completed_at=req.completed_at;
    state.complete_task(task_id,completed_at).await?;
    info!("Task completed");
    Ok(())
}
