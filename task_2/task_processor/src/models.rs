use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedTask {
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub value: Value,
}

#[derive(Serialize, Debug)]
pub struct ProcessedTask {
    pub uuid: Uuid,
    created_at: DateTime<Utc>,
    processed_at: DateTime<Utc>,
    description: Result<(), String>,
}

impl ProcessedTask {
    pub fn new(uuid: Uuid, created_at: DateTime<Utc>, description: Result<(), String>) -> Self {
        Self {
            uuid,
            created_at,
            processed_at: Utc::now(),
            description,
        }
    }
}
