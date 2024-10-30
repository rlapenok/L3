use std::{error::Error, sync::Arc};

use axum::async_trait;

use log::{debug, error, info, trace};
use sqlx::postgres::{PgListener, PgNotification};
use tokio::{
    select, spawn,
    sync::{mpsc::UnboundedSender, Mutex},
    task::JoinHandle
};
use tokio_util::sync::CancellationToken;


use crate::domain::{models::{Payload, TableChange}, table_change_listener::TableChangesListener};


#[derive(Clone)]
pub struct TableListener {
    listener: Arc<Mutex<PgListener>>,
    cancellation_token: Arc<CancellationToken>,
    sender: Arc<UnboundedSender<TableChange>>,
}

impl TableListener {
    pub fn new(
        listener: PgListener,
        cancellation_token: CancellationToken,
        sender: UnboundedSender<TableChange>,
    ) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            cancellation_token: Arc::new(cancellation_token),
            sender: Arc::new(sender),
        }
    }
    async fn recv(&self) -> Result<PgNotification, sqlx::Error> {
        let mut guard = self.listener.lock().await;
        guard.recv().await
    }

    async fn send_table_change_to_message_queue_sender(&self,payload:Payload,channel:&str,payload_for_send:&str){
        let task_id=payload.get_task_id();
        let table_change=TableChange::new(channel,payload,payload_for_send);
        if let Err(err)=self.sender.send(table_change){
            error!("TabelChangeListener::send_table_change_to_MessageQueueSender, task_id={:?} error: {}",task_id,err)
        }else {
        trace!("TabelChangeListener::send_table_change_to_MessageQueueSender, task_id={:?}",task_id);
        }    
    }
}
#[async_trait]
impl TableChangesListener for TableListener {
    fn run_listener(&self) -> JoinHandle<()> {
        let listener = self.clone();
         spawn(async move {
            loop {
                select! {
                    _ = listener.cancellation_token.cancelled()=>{
                        debug!("Handle stop signal");
                        break
                    }
                    result=listener.recv()=>{
                        match result {
                            Err(err)=>{
                                error!("TabelChangeListener: error while recv change from database:{}",err)
                            }
                            Ok(table_change)=>{
                                debug!("TabelChangeListener: new table change");
                                match serde_json::from_str::<Payload>(table_change.payload()) {
                                    Ok(payload)=>{
                                        listener.send_table_change_to_message_queue_sender(payload,table_change.channel(),table_change.payload()).await;
                                    }
                                    Err(err)=>{
                                        error!("Error while deserialize change from database:{}",err)
                                    }
                                }
                            }
                        }
                    }
                    
                    }
                }
            })
    }
    async fn stop_listener(&self) -> Result<(), Box<dyn Error>> {
        let mut guard = self.listener.lock().await;
        guard.unlisten_all().await.inspect_err(|err| {
            error!(
                "Error while TableChangesListener unlisten channels: {}",
                err
            );
        })?;
        self.sender.closed().await;
        info!("TableChangesListener - STOP");
        Ok(())
    }
}