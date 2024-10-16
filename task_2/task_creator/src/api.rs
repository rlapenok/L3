use axum::extract::State;
use serde_json::Value;

use crate::{domain::models::Task, errors::ServerError, request_models::TaskExtractor, state::ServerState};

pub async fn save_task(State(state):State<ServerState>,TaskExtractor(task):TaskExtractor<Value>)->Result<(),ServerError> {
    let task=Task::from(task);
    state.save_task(task).await?;
    Ok(())

}
