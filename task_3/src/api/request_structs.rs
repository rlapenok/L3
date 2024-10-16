use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::Deserialize;
use validator::Validate;

use crate::errors::ServerErrors;

#[derive(Deserialize, Validate)]
pub struct JoinLeaveGetMessagesRoom {
    #[validate(length(min = 1))]
    pub user_name: String,
    #[validate(length(min = 1))]
    pub room_name: String,
}

#[async_trait]
impl<S> FromRequest<S> for JoinLeaveGetMessagesRoom
where
    S: Send + Sync,
{
    type Rejection = ServerErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json_request = Json::<Self>::from_request(req, state).await?;
        json_request.0.validate()?;
        Ok(json_request.0)
    }
}

#[derive(Deserialize, Validate)]
pub struct SendMessage {
    #[validate(length(min = 1))]
    pub room_name: String,
    #[validate(length(min = 1))]
    pub user_name: String,
    #[validate(length(min = 1))]
    pub data: String,
}

#[async_trait]
impl<S> FromRequest<S> for SendMessage
where
    S: Send + Sync,
{
    type Rejection = ServerErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json_request = Json::<Self>::from_request(req, state).await?;
        json_request.0.validate()?;
        Ok(json_request.0)
    }
}
