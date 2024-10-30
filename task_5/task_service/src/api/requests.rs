use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};
use validator::Validate;

use crate::errors::ServerError;

#[derive(Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1))]
    pub description: String,
}


#[derive(Deserialize,Validate)]
pub struct UpdateTask{
   pub completed_at:DateTime<Utc>
}

pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for JsonExtractor<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let data = Json::<T>::from_request(req, state).await?;
        data.0.validate()?;
        Ok(Self(data.0))
    }
}
