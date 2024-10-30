use axum::extract::State;

use tracing::{ info, instrument, Span};

use crate::{
    api::response::CreatedTask,
    domain::{models::NewTask, task_service::TaskService},
    errors::ServerError,
};

use super::requests::{CreateTaskRequest, JsonExtractor};

#[instrument(name = "create_task_handler", skip_all,fields(task_id))]
pub async fn create_task<T>(
    State(state): State<T>,
    JsonExtractor(req): JsonExtractor<CreateTaskRequest>,
) -> Result<CreatedTask, ServerError>
where
    T: TaskService,
{
    let new_task = NewTask::from(req);
    let task_id = new_task.id;
    Span::current().record("task_id", task_id.to_string());
    state.create_task(new_task).await?;
    let response = CreatedTask { id: task_id };
    info!("Successfully");
    Ok(response)
}
