use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use tokio_postgres::error::SqlState;
use validator::ValidationErrors;

#[derive(Serialize, Error,Debug)]
pub enum ServerErrors {
    #[error("error while deserialize data from request:{0}")]
    DeserializeError(String),
    #[error("error while deserialize data from request:{0}")]
    ValidationErrors(#[from] ValidationErrors),
    #[error("error while interact with postgres :{0}")]
    PostgresError(String),
    #[error("error while create jwt:{0}")]
    CreateTokenError(String),
    #[error("error while parse jwt:{0}")]
    ParseTokenError(String),
    #[error("error while parse uuid:{0}")]
    ParseUuidError(String),
    #[error("error while login:{0}")]
    ErrUniqueLogin(String),
    #[error("not found post")]
    NotFindPost,
    #[error("user not found")]
    NotFindUser,
    #[error("missing authorization header")]
    MissingAuthorizationHeader,
    #[error("missing bearer in authorization header")]
    MissingBearer,
}

impl From<tokio_postgres::Error> for ServerErrors {
    fn from(value: tokio_postgres::Error) -> Self {
        if let Some(sql_state) = value.code() {
            match *sql_state {
                SqlState::UNIQUE_VIOLATION => return Self::ErrUniqueLogin(value.to_string()),
                _ => {
                    return Self::PostgresError(value.to_string());
                }
            }
        }
        Self::PostgresError(value.to_string())
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for ServerErrors {
    fn from(value: bb8::RunError<tokio_postgres::Error>) -> Self {
        Self::PostgresError(value.to_string())
    }
}

impl From<uuid::Error> for ServerErrors {
    fn from(value: uuid::Error) -> Self {
        Self::ParseUuidError(value.to_string())
    }
}


impl IntoResponse for ServerErrors {
    fn into_response(self) -> Response {
        match self {
            Self::DeserializeError(err) => (StatusCode::BAD_REQUEST, err).into_response(),
            Self::ValidationErrors(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
            Self::PostgresError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
            Self::CreateTokenError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
            Self::ParseTokenError(err)=>(StatusCode::UNAUTHORIZED,err).into_response(),
            Self::ParseUuidError(err) => (StatusCode::BAD_REQUEST, err).into_response(),
            Self::ErrUniqueLogin(err) => (StatusCode::CONFLICT, Json(err)).into_response(),
            Self::NotFindPost => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::NotFindUser=>(StatusCode::UNAUTHORIZED,self.to_string()).into_response(),
            Self::MissingAuthorizationHeader=>(StatusCode::UNAUTHORIZED,self.to_string()).into_response(),
            Self::MissingBearer=>(StatusCode::UNAUTHORIZED,self.to_string()).into_response(),
        }
    }
}
