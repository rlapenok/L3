use std::{error::Error, sync::Arc};

use axum::async_trait;
use sqlx::postgres::{PgListener, PgNotification};
use tokio::{
    select, spawn,
    sync::{mpsc::UnboundedSender, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::domain::change_notifier::TableChangeNotifier;

#[derive(Clone)]
pub struct Notifier {
    listener: Arc<Mutex<PgListener>>,
    cancellation_token: Arc<CancellationToken>,
    change_sender: Arc<UnboundedSender<()>>,
}
impl Notifier {
    pub(crate) fn new(
        listener: PgListener,
        cancellation_token: CancellationToken,
        change_sender: UnboundedSender<()>,
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
    async fn run_notifier(&self) -> JoinHandle<()> {
        let notifier = self.clone();
        spawn(async move {
            println!("Start notifier");
            loop {
                select! {
                    result= notifier.recv()=>{
                        println!("{:?}",result);
                        //TODO send message to kafka producer
                    }
                    _= notifier.cancellation_token.cancelled()=>{
                        println!("Notifier handle ctrl c");
                        break;
                    }
                }
            }
            println!("Stop notifier task")
        })
    }
    async fn stop_notifier(&self) -> Result<(), Box<dyn Error>> {
        let mut guard = self.listener.lock().await;
        guard.unlisten_all().await?;
        self.change_sender.closed().await;
        Ok(())
    }
}
