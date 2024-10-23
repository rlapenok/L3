use std::{sync::Arc, time::Duration};

use axum::async_trait;
use log::{error, info};
use rdkafka::{
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
};
use tokio::{
    select, spawn,
    sync::{mpsc::UnboundedReceiver, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::domain::{change_notifier::TableChanges, kafka_sender::KafkaSender};

#[derive(Clone)]
pub struct KafkaProducer {
    topic_name: Arc<String>,
    producer: FutureProducer,
    receiver: Arc<Mutex<UnboundedReceiver<TableChanges>>>,
    cancellation_token: Arc<CancellationToken>,
}

impl KafkaProducer {
    pub fn new(
        topic_name: String,
        producer: FutureProducer,
        receiver: UnboundedReceiver<TableChanges>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            topic_name: Arc::new(topic_name),
            producer,
            receiver: Arc::new(Mutex::new(receiver)),
            cancellation_token: Arc::new(cancellation_token),
        }
    }
    async fn recv_message(&self) -> Option<TableChanges> {
        let mut guard = self.receiver.lock().await;
        guard.recv().await
    }
}

#[async_trait]
impl KafkaSender for KafkaProducer {
    fn run_sender(&self) -> JoinHandle<()> {
        let producer = self.clone();

        spawn(async move {
            info!("Start KafkaSender");

            loop {
                select! {
                    result=producer.recv_message()=>{
                        if let Some(notify)=result{
                            let kafka_message=FutureRecord::to(&producer.topic_name).key(&notify.table_name).payload(&notify.payload);
                            if let Err(err) =producer.producer.send(kafka_message, Timeout::After(Duration::from_secs(0))).await  {
                                error!("Error while send message to kafka:{:?}",err)
                            }
                        }
                    }
                    _ = producer.cancellation_token.cancelled()=>{
                        break
                    }
                }
            }
        })
    }
    async fn stop_sender(&self) {
        let mut guard = self.receiver.lock().await;
        guard.close();
        while let Some(notify) = guard.recv().await {
            let kafka_message = FutureRecord::to(&self.topic_name)
                .key(&notify.table_name)
                .payload(&notify.payload);
            if let Err(err) = self
                .producer
                .send(kafka_message, Timeout::After(Duration::from_secs(0)))
                .await
            {
                error!("Error while send message to kafka:{:?}", err)
            }
        }
        if let Err(err) = self.producer.flush(Duration::from_secs(10)) {
            error!("Error while flush kafka producer :{}", err)
        }
        info!("Stop KafkaSender")
    }
}
