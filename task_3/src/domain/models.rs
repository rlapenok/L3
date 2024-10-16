use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::request_structs::{JoinLeaveGetMessagesRoom, SendMessage};

#[derive(Serialize, Deserialize)]
pub struct Message {
    room_name: Arc<String>,
    user_name: String,
    data: String,
    date: DateTime<Utc>,
}

impl From<SendMessage> for Message {
    fn from(value: SendMessage) -> Self {
        Self {
            room_name: Arc::new(value.room_name),
            user_name: value.user_name,
            data: value.data,
            date: Utc::now(),
        }
    }
}
#[typetag::serde]
impl LikeMessage for Message {
    fn get_room_name(&self) -> Arc<String> {
        self.room_name.clone()
    }
    fn get_user_name(&self) -> &str {
        &self.user_name
    }
}

#[typetag::serde(tag = "type", content = "value")]
pub trait LikeMessage: Send + Sync {
    fn get_room_name(&self) -> Arc<String>;
    fn get_user_name(&self) -> &str;
}

pub struct GetMessage {
    room_name: Arc<String>,
    user_name: String,
}

impl GetMessage {
    pub fn get_room_name(&self) -> Arc<String> {
        self.room_name.clone()
    }
    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}

impl From<JoinLeaveGetMessagesRoom> for GetMessage {
    fn from(value: JoinLeaveGetMessagesRoom) -> Self {
        Self {
            room_name: Arc::new(value.room_name),
            user_name: value.user_name,
        }
    }
}
