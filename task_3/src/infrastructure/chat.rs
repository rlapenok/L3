use std::sync::Arc;

use dashmap::DashMap;
use log::{debug, info};
use tokio::{
    select, spawn,
    sync::{mpsc::unbounded_channel, RwLock},
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::domain::models::LikeMessage;

use super::{errors::Errors, room::Room};

pub struct Chat {
    rooms: DashMap<Arc<String>, Room>,
    cancellation_token: CancellationToken,
    task_tracker: TaskTracker,
}

impl Chat {
    pub fn new() -> Self {
        let rooms = DashMap::default();
        let cancellation_token = CancellationToken::new();
        let task_tracker = TaskTracker::new();
        Self {
            rooms,
            cancellation_token,
            task_tracker,
        }
    }

    pub fn join_room(&self, user_name: String, room_name: String) -> Result<(), Errors> {
        if let Some(room) = self.rooms.get(&room_name) {
            room.join_new_user(user_name)?;
            return Ok(());
        }
        let cancellation_token = self.cancellation_token.child_token();
        let room_name = Arc::new(room_name);
        let (sender, mut receiver) = unbounded_channel();
        let messages = Arc::new(RwLock::new(Vec::new()));
        let messages_for_rcv_message_handle = messages.clone();
        let rooom_name_for_rcv_messages_handle = room_name.clone();
        let room_recv_message_handle = spawn(async move {
            info!(
                "Room: {} ----> Start receive messages",
                rooom_name_for_rcv_messages_handle
            );
            loop {
                select! {
                 result=receiver.recv()=>{
                        if let Some(message)=result{
                            debug!("Room:{} ----> receive new message",rooom_name_for_rcv_messages_handle);
                           let mut guard=messages_for_rcv_message_handle.write().await;
                            guard.push(message);
                        }else {
                            info!("Room: {} ----> End receive messages",rooom_name_for_rcv_messages_handle);
                            break;
                        }

                 }
                _= cancellation_token.cancelled()=>{
                    info!("Room: {} ----> is closed",rooom_name_for_rcv_messages_handle);
                    receiver.close();
                    break
                }
                     }
            }
        });
        self.task_tracker.spawn(room_recv_message_handle);
        let room = Room::new(room_name.clone(), sender, messages);
        room.join_new_user(user_name)?;
        self.rooms.insert(room_name, room);
        Ok(())
    }
    pub fn leave_room(&self, user_name: String, room_name: String) -> Result<(), Errors> {
        if let Some(room) = self.rooms.get(&room_name) {
            return room.leave(user_name);
        }
        Err(Errors::RoomNotExist)
    }
    pub fn send_message(&self, message: Box<dyn LikeMessage>) -> Result<(), Errors> {
        if let Some(room) = self.rooms.get(&message.get_room_name()) {
            room.send_message(message)?;
            return Ok(());
        }
        Err(Errors::RoomNotExist)
    }
    pub async fn get_messages(
        &self,
        room_name: Arc<String>,
        user_name: &str,
    ) -> Result<String, Errors> {
        if let Some(room) = self.rooms.get(&room_name) {
            let messages = room.get_messages(user_name).await?;
            return Ok(messages);
        }
        Err(Errors::RoomNotExist)
    }
    pub async fn close_chat(&self) {
        self.task_tracker.close();
        self.cancellation_token.cancel();
        for room in &self.rooms {
            room.close().await;
        }
        self.task_tracker.wait().await;
    }
}
