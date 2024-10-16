use std::io;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error,Debug,Serialize)]
#[serde(tag="error",content="message")]
pub enum ServerError{
    #[error("Error while read/lock/unlock file: {0}")]
    InternalError(String),
    #[error("Error while serialize task: {0}")]
    SerializeError(String),
}

#[derive(Error,Debug)]
pub enum CliError{
    #[error("The path to the directory contains the file: {0}")]
    NotDirectory(String)
}


impl From <io::Error> for ServerError{
    fn from(value: io::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<serde_json::Error> for ServerError{
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeError(value.to_string())
    }
}

impl IntoResponse for ServerError{
    fn into_response(self)->Response{
            (StatusCode::INTERNAL_SERVER_ERROR,Json(self)).into_response()
    }
}