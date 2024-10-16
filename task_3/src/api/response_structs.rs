use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetMessagesReposne {
    pub room_name: Arc<String>,
    pub messages: String,
}

impl IntoResponse for GetMessagesReposne {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
