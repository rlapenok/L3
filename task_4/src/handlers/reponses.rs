use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUserProductResponse {
    pub id: Uuid,
}

impl IntoResponse for CreateUserProductResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
