use axum::{extract::rejection::JsonRejection, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug,Serialize,Error)]
pub enum ServerError{
    #[error("Error while deserialize request:{0}")]
    Deserialize(String),
    #[error("Error while vaidate request:{0}")]
    Validation(String),
}

impl From<JsonRejection> for ServerError{
    fn from(value: JsonRejection) -> Self {
        Self::Deserialize(value.to_string())
    }
}


impl From<validator::ValidationErrors> for ServerError{
    fn from(value: validator::ValidationErrors) -> Self {
        Self::Validation(value.to_string())
    }
}


impl IntoResponse for  ServerError{
    fn into_response(self) -> Response {
        match self {
            Self::Deserialize(_)=>{
                (StatusCode::BAD_REQUEST,Json(self)).into_response()
            },
            Self::Validation(_)=>{
                (StatusCode::BAD_REQUEST,Json(self)).into_response()
            }
        }
    }
}