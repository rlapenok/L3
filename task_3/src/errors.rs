use std::fmt::Debug;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

use crate::infrastructure::errors::Errors;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "error", content = "description")]
pub enum ServerErrors {
    #[error("Error while deserialize request:{0}")]
    Deserialize(String),
    #[error("Error while validation request:{0}")]
    Validation(String),
    #[error("Error while validation request:{0}")]
    Chat(#[from] Errors),
}

impl IntoResponse for ServerErrors {
    fn into_response(self) -> Response {
        match &self {
            Self::Deserialize(_) => {
                println!("{}", self);
                (StatusCode::BAD_REQUEST, Json(self)).into_response()
            }
            Self::Validation(_) => {
                println!("{:?}", self);
                (StatusCode::BAD_REQUEST, Json(self)).into_response()
            }
            Self::Chat(_) => (StatusCode::BAD_REQUEST, Json(self)).into_response(),
        }
    }
}

impl From<JsonRejection> for ServerErrors {
    fn from(value: JsonRejection) -> Self {
        Self::Deserialize(value.to_string())
    }
}

impl From<ValidationErrors> for ServerErrors {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value.to_string())
    }
}
