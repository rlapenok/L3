use chrono::{DateTime, Utc};
use redis_macros::FromRedisValue;
use serde::Deserialize;
use uuid::Uuid;


#[allow(dead_code)]
#[derive(Deserialize,FromRedisValue)]
pub struct Task {
    operation: String,
    pub id: Uuid,
    description: String,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    trace_id: String,
    span_id: String,
}
