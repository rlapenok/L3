use serde::Serialize;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error, Serialize)]
pub enum Errors {
    #[error("user exist in room")]
    UserExist,
    #[error("user not exist in room")]
    UserNotExist,
    #[error("room not exist in chat")]
    RoomNotExist,
    #[error("internal Sender is not in the chat:{0}")]
    SendMessage(String),
    #[error("error while serialize messages:{0}")]
    SerializeMessages(String),
}

impl<T> From<SendError<T>> for Errors {
    fn from(value: SendError<T>) -> Self {
        Self::SendMessage(value.to_string())
    }
}
impl From<serde_json::Error> for Errors {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeMessages(value.to_string())
    }
}
