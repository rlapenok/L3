use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use tokio::sync::mpsc::{error::SendError, UnboundedSender};

use crate::domain::models::LikeMessage;

pub struct User {
    sender: Arc<UnboundedSender<Box<dyn LikeMessage>>>,
    offset: Arc<AtomicUsize>,
}

impl User {
    pub fn new(sender: Arc<UnboundedSender<Box<dyn LikeMessage>>>) -> Self {
        Self {
            sender,
            offset: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn send_message(
        &self,
        message: Box<dyn LikeMessage>,
    ) -> Result<(), SendError<Box<dyn LikeMessage>>> {
        self.sender.send(message)
    }
    pub fn get_offset(&self) -> usize {
        self.offset.load(Ordering::Relaxed)
    }
    pub fn update_offset(&self,offet:usize) {
        self.offset.store(offet, Ordering::Relaxed);
    }
}
