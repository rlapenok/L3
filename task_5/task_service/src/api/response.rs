use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::models::Task;

#[derive(Serialize)]
pub struct CreatedTask {
    pub id: Uuid,
}

impl IntoResponse for CreatedTask {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct GetTask {
    task_info: Task,
}

impl From<Task> for GetTask {
    fn from(value: Task) -> Self {
        Self { task_info: value }
    }
}

impl IntoResponse for GetTask {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
