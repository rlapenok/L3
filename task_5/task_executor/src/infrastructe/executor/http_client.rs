use std::sync::Arc;

use chrono::{DateTime, Utc};
use reqwest::{Client, Error, Response};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Body {
    pub completed_at: DateTime<Utc>,
}

#[derive(Clone)]
pub(crate) struct HttpClient {
    client: Client,
    url: Arc<str>,
}
impl HttpClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: Arc::from(url),
        }
    }

    pub async fn send_complete_task(
        &self,
        id: Uuid,
        completed_at: DateTime<Utc>,
    ) -> Result<Response, Error> {
        let url = format!("{}/tasks/{}/complete", self.url, id);
        self.client
            .post(url)
            .json(&Body { completed_at })
            .send()
            .await
    }
}
