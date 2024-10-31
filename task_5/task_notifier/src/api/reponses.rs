use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::models::Task;

#[derive(Serialize)]
pub struct Notification {
    tasks: Vec<Task>,
}

impl Notification {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}

impl IntoResponse for Notification {
    fn into_response(self) -> Response {
        if !self.tasks.is_empty()  {
          return   (StatusCode::OK, Json(self)).into_response()   
        }
        (StatusCode::NO_CONTENT).into_response()


    }
}
