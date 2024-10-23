use std::{error::Error, sync::Arc};

use axum::async_trait;
use log::{error, info};
use sqlx::postgres::{PgListener, PgNotification};
use tokio::{
    select, spawn,
    sync::{mpsc::UnboundedSender, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::domain::change_notifier::{TableChangeNotifier, TableChanges};

#[derive(Clone)]
pub struct Notifier {
    listener: Arc<Mutex<PgListener>>,
    cancellation_token: Arc<CancellationToken>,
    change_sender: Arc<UnboundedSender<TableChanges>>,
}
impl Notifier {
    pub(crate) fn new(
        listener: PgListener,
        cancellation_token: CancellationToken,
        change_sender: UnboundedSender<TableChanges>,
    ) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            cancellation_token: Arc::new(cancellation_token),
            change_sender: Arc::new(change_sender),
        }
    }
    async fn recv(&self) -> Result<PgNotification, sqlx::Error> {
        let mut guard = self.listener.lock().await;
        guard.recv().await
    }
}

#[async_trait]
impl TableChangeNotifier for Notifier {
    fn run_notifier(&self) -> JoinHandle<()> {
        let notifier = self.clone();
        spawn(async move {
            info!("Stop TableChangeNotifier");

            loop {
                select! {
                    result= notifier.recv()=>{
                        match result{
                            Ok(notification)=>{

                                let changes=TableChanges{
                                    table_name:notification.channel().to_owned(),
                                    payload:notification.payload().to_owned()
                                };
                                if let Err(err)=notifier.change_sender.send(changes){
                                    error!("Error while send message to kafka_producer:{}",err)
                                }
                            }
                            Err(err)=>{
                                error!("Error while recv notification from database:{}",err)
                            }
                        }
                    }
                    _= notifier.cancellation_token.cancelled()=>{
                        break;
                    }
                }
            }
        })
    }
    async fn stop_notifier(&self) -> Result<(), Box<dyn Error>> {
        let mut guard = self.listener.lock().await;
        guard.unlisten_all().await?;
        self.change_sender.closed().await;
        info!("Stop TableChangeNotifier");
        Ok(())
    }
}
