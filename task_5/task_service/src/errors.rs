use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

use crate::domain::service_error::TaskServiceError;

#[derive(Debug, Serialize, Error)]
#[serde(tag = "type", content = "description")]
pub enum ServerError {
    #[error("Error while deserialize request:{0}")]
    Deserialize(String),
    #[error("Error while vaidate request:{0}")]
    Validation(String),
    #[error("Error while vaidate request:{0}")]
    TaskServiceError(#[from] TaskServiceError),
}

impl From<JsonRejection> for ServerError {
    fn from(value: JsonRejection) -> Self {
        Self::Deserialize(value.to_string())
    }
}

impl From<validator::ValidationErrors> for ServerError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::Validation(value.to_string())
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match &self {
            Self::Deserialize(_) => (StatusCode::BAD_REQUEST, Json(self)).into_response(),
            Self::Validation(_) => (StatusCode::BAD_REQUEST, Json(self)).into_response(),
            Self::TaskServiceError(err) => match err {
                TaskServiceError::TaskExist(_) => {
                    (StatusCode::CONFLICT, Json(self)).into_response()
                }
                TaskServiceError::OtherError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
                }
                TaskServiceError::TaskNotFound(_) => {
                    (StatusCode::NOT_FOUND, Json(self)).into_response()
                }
            },
        }
    }
}
