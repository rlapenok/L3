use serde::Serialize;
use thiserror::Error;


pub enum RepoType {
    Products,
    Users,
}

#[derive(Debug, Error, Serialize)]
#[serde(tag="type",content="desc")]
pub enum RepoError {
    #[error("user/product not exist")]
    IdNotExist,
    #[error("email exist in database; description: {0}")]
    UserEmailExist(String),
    #[error("product exist in database; description: {0}")]
    ProductNameExist(String),
    #[error("error whhile execute request in database: {0}")]
    Internal(String),
}

impl RepoError {
    pub fn from(sqlx_err: sqlx::Error, repo_type: RepoType) -> Self {
        if let Some(err) = sqlx_err.as_database_error() {
            if err.is_unique_violation() {
                match repo_type {
                    RepoType::Products => return Self::ProductNameExist(err.to_string()),
                    _ => return Self::UserEmailExist(err.to_string()),
                }
            }
            return Self::Internal(err.to_string());
        }
        return Self::Internal(sqlx_err.to_string());
    }
}
