use axum::{
    http::{header::AUTHORIZATION, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RegisterResponse(pub Uuid, pub String);

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> Response {
        let header = (AUTHORIZATION, format!("Bearer {}", self.1));
        (StatusCode::OK, [header], Json(self.0)).into_response()
    }
}

pub struct LoginResponse(pub String);
impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        let header = (AUTHORIZATION, format!("Bearer {}", self.0));
        (StatusCode::OK, [header]).into_response()
    }
}

#[derive(Serialize)]
pub struct CreatePostResponse{
    pub post_uid:Uuid
}

impl IntoResponse for CreatePostResponse{
    fn into_response(self) -> Response {
        (StatusCode::CREATED,Json(self)).into_response()
    }
}
