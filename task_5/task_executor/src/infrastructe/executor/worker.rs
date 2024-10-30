use std::{future::Future, pin::Pin, sync::Arc};

use crate::domain::{models::Task, task_executor::Job};
use flume::Receiver;

use tokio::{
    select, spawn, task::{JoinError, JoinHandle}
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{ debug, error,  info_span, instrument, Instrument, Span};

pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Receiver<Task>,
        cancellation_token: CancellationToken,
        job: Job,
    ) -> Self {
        let task_tracker=TaskTracker::new();
        let receiver = Arc::new(receiver);
        let job=Arc::new(job);
        let task = spawn(async move {
            loop {
                select! {
                    _ = cancellation_token.cancelled()=>{
                            break
                    }
                    _= recv_task(id,receiver.clone(), job.clone(),&task_tracker).instrument(info_span!("Worker"))=>{

                    }
                }
            }
           task_tracker.close();
           task_tracker.wait().await;
        });
        Self { handle: task }
    }
}

#[instrument(skip_all,name="get_tasks_from_redis_receiver",fields(task_id,worker_id=id))]
async fn recv_task(id:usize,receiver:Arc<Receiver<Task>>,job:Arc<Job>,task_tracker:&TaskTracker){
        match receiver.recv_async().await {
            Ok(task)=>{
                let span=Span::current();
                span.record("task_id", task.id.to_string());
                debug!("new task");
                task_tracker.spawn(job(task));
            }
            Err(err)=>{
            error!(" {}", err);
            }
        }
}


impl Future for Worker {
    type Output = Result<(), JoinError>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.get_mut().handle).poll(cx)
    }
}
