use std::sync::Arc;

use crate::{
    domain::models::{GetMessage, LikeMessage},
    errors::ServerErrors,
};

use super::chat::Chat;

#[derive(Clone)]
pub struct ServerState {
    chat: Arc<Chat>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            chat: Arc::new(Chat::new()),
        }
    }

    pub fn join_room(&self, user_name: String, room_name: String) -> Result<(), ServerErrors> {
        self.chat.join_room(user_name, room_name)?;
        Ok(())
    }
    pub fn leave_room(&self, user_name: String, room_name: String) -> Result<(), ServerErrors> {
        self.chat.leave_room(user_name, room_name)?;
        Ok(())
    }

    pub fn send_message(&self, message: Box<dyn LikeMessage>) -> Result<(), ServerErrors> {
        self.chat.send_message(message)?;
        Ok(())
    }

    pub async fn get_messages(&self, get_messages: GetMessage) -> Result<String, ServerErrors> {
        let messages = self
            .chat
            .get_messages(get_messages.get_room_name(), get_messages.get_user_name())
            .await?;
        Ok(messages)
    }

    pub async fn shutdown(&self) {
        self.chat.close_chat().await;
    }
}
