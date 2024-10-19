use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::server_errors::ServerError;

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