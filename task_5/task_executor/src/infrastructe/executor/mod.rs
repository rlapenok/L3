mod http_client;
mod worker;

use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use flume::Receiver;
use http_client::HttpClient;
use log::info;
use tokio::time::sleep;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info_span,  Instrument, Span};
use worker::Worker;

use crate::domain::{
    models::Task,
    task_executor::{Job, TaskExecutor},
};

pub struct Executor {
    num_workers: usize,
    receiver: Receiver<Task>,
    workers: TaskTracker,
    client: HttpClient,
}

impl Executor {
    pub fn new(num_workers: usize, receiver: Receiver<Task>, url: &str) -> Self {
        let workers = TaskTracker::new();
        let client = HttpClient::new(url);
        Self {
            num_workers,
            workers,
            receiver,
            client,
        }
    }
}

#[async_trait]
impl TaskExecutor for Executor {
    fn run(&self, cancellation_token: CancellationToken) {
        (1..=self.num_workers).for_each(|id| {
            let job = self.do_job();
            let worker = Worker::new(id, self.receiver.clone(), cancellation_token.clone(), job);
            self.workers.spawn(worker);
        });
        self.workers.close();
    }
    fn do_job(&self) -> Job {
        let http_client = self.client.clone();
        Box::new(move |task: Task| {
            let http_client = http_client.clone();
            Box::pin(async move {
                //imitation of work
                sleep(Duration::from_millis(200)).await;
                let span=Span::current();
                span.record("task_id", task.id.to_string());
                let completed_at = Utc::now();
                 if let Err(err) =http_client.send_complete_task(task.id, completed_at).await  {
                    error!("{}",err);
                }else {
                    info!("ok")
                }

            }.instrument(info_span!("do_work")))
        })
    }
    async fn stop(self) {
        let recv = self.receiver;
        drop(recv);
        self.workers.wait().await;
    }
}
