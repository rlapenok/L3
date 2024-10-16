use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Task{
    pub uuid:Uuid,
    created_at:DateTime<Utc>,
    value:Value
}
impl From<Value> for Task{
    fn from(value: Value) -> Self {
        Self { uuid: Uuid::new_v4(),created_at:Utc::now() ,value }
    }
}
