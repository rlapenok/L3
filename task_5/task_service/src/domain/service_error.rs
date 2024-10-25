use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TaskServiceError {
    #[error("{0}")]
    TaskExist(String),
    #[error("{0}")]
    TaskNotFound(String),
    #[error("{0}")]
    OtherError(String),
}

impl From<sqlx::Error> for TaskServiceError {
    fn from(value: sqlx::Error) -> Self {
        if let Some(err) = value.as_database_error() {
            if err.is_unique_violation() {
                Self::TaskExist(err.to_string())
            } else {
                Self::OtherError(err.to_string())
            }
        } else {
            match value {
                sqlx::Error::RowNotFound => Self::TaskNotFound(value.to_string()),
                _ => Self::OtherError(value.to_string()),
            }
        }
    }
}
