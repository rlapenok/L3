use chrono::{DateTime, Utc};
use redis_macros::FromRedisValue;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

pub type Message = Vec<(String, Vec<(String, Vec<(String, Task)>)>)>;

#[derive(Deserialize)]
pub enum TypeNotification {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "non_completed")]
    NonCompleted,
}

#[derive(Deserialize, Serialize, FromRedisValue, Debug)]
pub struct Task {
    operation: String,
    pub id: Uuid,
    description: String,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    trace_id: String,
    span_id: String,
}

#[derive(Serialize, Deserialize,Debug,Default)]
pub struct LastTasks {
    completed: String,
    non_completed: String,
}
impl LastTasks{
    pub fn new(completed:String,non_completed:String)->Self{
        Self { completed, non_completed }
    }
}

#[derive(Default)]
pub struct LastTasksRedis {
    
    completed: Mutex<String>,
    non_completed: Mutex<String>,
}

impl LastTasksRedis{
    pub async fn get_completed(&self)->String

    {
        let guard=self.completed.lock().await;
        guard.clone()
    }
    pub async fn get_non_compeleted(&self)->String{
        let guard=self.non_completed.lock().await;
        guard.clone()
    }
    pub async fn update_completed(&self,data:String){
        let mut guard=self.completed.lock().await;
        *guard=data;
    }
    pub async fn update_non_completed(&self,data:String){
        let mut guard=self.non_completed.lock().await;
        *guard=data;
    }
}

impl From<LastTasks> for LastTasksRedis{
    fn from(value: LastTasks) -> Self {
        Self { completed: Mutex::new(value.completed), non_completed: Mutex::new(value.non_completed) }
        
    }
}
