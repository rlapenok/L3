use std:: sync::Arc;

use axum::async_trait;
use log::{debug, error, info, trace};
use redis::{AsyncCommands, Client, RedisResult};
use tokio::{
    select, spawn,
    sync::{mpsc::UnboundedReceiver, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::domain::{message_queue_sender::MessageQueueSender, models::TableChange};

#[derive(Clone)]
pub struct RedisClient {
    client: Arc<Client>,
    cancellation_token: Arc<CancellationToken>,
    recv: Arc<Mutex<UnboundedReceiver<TableChange>>>,
}

impl RedisClient {
    pub fn new(
        client: Client,
        cancellation_token: CancellationToken,
        recv: UnboundedReceiver<TableChange>,
    ) -> Self {
        Self {
            client: Arc::new(client),
            cancellation_token: Arc::new(cancellation_token),
            recv: Arc::new(Mutex::new(recv)),
        }
    }
    async fn read_messages(&self) -> RedisResult<()> {
        let mut guard = self.recv.lock().await;
        while let Some(table_change) = guard.recv().await {
            trace!("MessageQueueSender::recv_tabel_change_from_TableChangeListener, task_id={}",table_change.get_task_id());
            self.send_to_queue(table_change).await?;
    }
    Ok(())
}
    async fn send_to_queue(&self,table_change:TableChange)->RedisResult<()>{
        let mut connection=self.client.get_multiplexed_tokio_connection().await.inspect_err(|err|{
            error!("MessageQueueSender::send_to_queue, task_id={} error: {}",table_change.get_task_id(),err);
        })?;
        connection.xadd::<&str, &str, &str, &str, String>(& *table_change.channel, "*", &[("task",&table_change.payload_for_send)]).await.inspect_err(|err|{
            error!("MessageQueueSender::send_to_queue, task_id={} error: {}",table_change.get_task_id(),err);
        })?;
        debug!("MessageQueueSender::send_to_queue, task_id={}",table_change.get_task_id());
        
        Ok(())
    }
}

#[async_trait]
impl MessageQueueSender for RedisClient {
    fn run_sender(&self) -> JoinHandle<()> {
        let sender = self.clone();
        spawn(async move {
            select! {
                _ = sender.read_messages()=>{}

                _ = sender.cancellation_token.cancelled()=>{
                    debug!("Handle stop signal")
                }
            }
        })

    }
    async fn stop_sender(&self) {
        let mut recv = self.recv.lock().await;
        recv.close();
        info!("MessageQueueSender - STOP")
    }
}
