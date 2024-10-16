use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use dashmap::{DashMap, Entry};
use log::info;

use tokio::sync::{mpsc::UnboundedSender, RwLock};

use crate::domain::models::LikeMessage;

use super::{errors::Errors, user::User};

#[derive(Clone)]
pub struct Room {
    name: Arc<String>,
    sender: Arc<UnboundedSender<Box<dyn LikeMessage>>>,
    user_count: Arc<AtomicUsize>,
    messages: Arc<RwLock<Vec<Box<dyn LikeMessage>>>>,
    users: Arc<DashMap<String, User>>,
}

impl Room {
    pub fn new(
        name: Arc<String>,
        sender: UnboundedSender<Box<dyn LikeMessage>>,
        messages: Arc<RwLock<Vec<Box<dyn LikeMessage>>>>,
    ) -> Self {
        let users = Arc::new(DashMap::default());
        Self {
            name,
            sender: Arc::new(sender),
            user_count: Arc::new(AtomicUsize::new(0)),
            messages,
            users,
        }
    }

    pub fn join_new_user(&self, user_name: String) -> Result<(), Errors> {
        let arccount = Arc::strong_count(&self.users);
        info!("Users count:{}", arccount);
        if let Entry::Vacant(entry) = self.users.entry(user_name) {
            let user = User::new(self.sender.clone());
            entry.insert(user);
            self.user_count.fetch_add(1, Ordering::Relaxed);
            info!(
                "Room: {} ----> User count: {:?}",
                self.name,
                self.user_count.load(Ordering::SeqCst)
            );
            return Ok(());
        }
        Err(Errors::UserExist)
    }
    pub fn leave(&self, user_name: String) -> Result<(), Errors> {
        if let Some(user) = self.users.remove(&user_name) {
            drop(user);
            self.user_count.fetch_sub(1, Ordering::Relaxed);
            info!(
                "Room: {} ----> User count: {:?}",
                self.name,
                self.user_count.load(Ordering::SeqCst)
            );
            return Ok(());
        }
        Err(Errors::UserNotExist)
    }
    pub fn send_message(&self, message: Box<dyn LikeMessage>) -> Result<(), Errors> {
        if let Some(user) = self.users.get(message.get_user_name()) {
            user.send_message(message)?;
            return Ok(());
        }
        Err(Errors::UserNotExist)
    }
    pub async fn get_messages(&self, user_name: &str) -> Result<String, Errors> {
        if let Some(user) = self.users.get(user_name) {
            let user_offset = user.get_offset();
            let guard = self.messages.read().await;
            let offset=guard.len();
            let msg = guard
                .iter()
                .skip(user_offset)
                .collect::<Vec<&Box<dyn LikeMessage>>>();
            let messages = serde_json::to_string_pretty(&msg)?;
            user.update_offset(offset);
            return Ok(messages);
        }
        Err(Errors::UserNotExist)
    }
    pub async fn close(&self) {
        self.sender.closed().await;
    }
}
