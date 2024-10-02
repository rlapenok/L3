use axum::{
    async_trait, extract::{FromRequest, Request}, Json
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::errors::ServerErrors;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1))]
    pub login: String,
    #[validate(length(min = 1))]
    pub hashed_password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RegisterRequest
where
    S: Send + Sync,
{
    type Rejection = ServerErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let data = Json::<Self>::from_request(req, state)
            .await
            .map_err(|err| ServerErrors::DeserializeError(err.to_string()))?;
        let data = data.0;
        data.validate()?;
        Ok(data)
    }
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    pub user_uid: Uuid,
    #[validate(length(min = 1))]
    pub login: String,
    #[validate(length(min = 1))]
    pub hashed_password: String,
}

#[async_trait]
impl<S> FromRequest<S> for LoginRequest
where
    S: Send + Sync,
{
    type Rejection = ServerErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let data = Json::<Self>::from_request(req, state)
            .await
            .map_err(|err| ServerErrors::DeserializeError(err.to_string()))?;
        let data = data.0;
        data.validate()?;
        Ok(data)
    }
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub msg: String,
}


#[async_trait]
impl<S> FromRequest<S> for CreatePostRequest
where
    S: Send + Sync,
{
    type Rejection = ServerErrors;

    
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let data=Json::<Self>::from_request(req, state).await.map_err(|err|{
            ServerErrors::DeserializeError(err.to_string())
        })?;
        Ok(data.0)
    }
}
