use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::api::requests::CreateTaskRequest;

pub struct NewTask {
    pub id: Uuid,
    pub description: String,
    pub created_at:DateTime<Utc>,
    pub completed_at:Option<DateTime<Utc>>,
}

impl From<CreateTaskRequest> for NewTask {
    fn from(value: CreateTaskRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: value.description,
            created_at:Utc::now(),
            completed_at:None
        }
    }
}

#[derive(Serialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    description: String,
    created_at:DateTime<Utc>,
    completed_at:Option<DateTime<Utc>>,
}

#[derive(Serialize,Deserialize)]
pub struct Payload{
    id: Uuid,
    description: String,
    created_at:DateTime<Utc>,
    completed_at:Option<DateTime<Utc>>,
    trace_id:String,
    span_id:String
}
impl Payload{
    pub fn get_task_id(&self)->Uuid{
        self.id
    }
}

pub struct TableChange{
    pub channel:Arc<str>,
    pub payload:Payload,
    pub payload_for_send:Arc<str>,
}
 
 impl TableChange {
     pub fn new(channel:&str,payload:Payload ,payload_for_send:&str)->Self
     {
        Self { channel: Arc::from(channel), payload,payload_for_send:Arc::from(payload_for_send)}
     }
     pub fn get_task_id(&self)->Uuid{
        self.payload.id
     }
 }


 

