use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
pub struct TaskExtractor<J>(pub J);

#[async_trait]
impl<J,S> FromRequest<S> for TaskExtractor<J>
where
    S: Send + Sync,
    J:DeserializeOwned,
    Json<J>:FromRequest<S,Rejection = JsonRejection>
{
    type Rejection = Response;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<J>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?;
        Ok(Self(data))
    }
}

