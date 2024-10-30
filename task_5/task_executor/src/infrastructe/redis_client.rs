use std::sync::Arc;

use async_trait::async_trait;
use flume::Sender;
use redis::{
    streams::StreamReadOptions, AsyncCommands, Client as ClientRedis, FromRedisValue, RedisResult,
};
use tokio::{select, spawn, sync::Mutex, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, instrument, trace, Instrument, Span};

use crate::domain::{models::Task, redis_receiver::RedisReceiver};

#[derive(Clone)]
pub struct RedisClient {
    redis_client: Arc<ClientRedis>,
    sender: Arc<Sender<Task>>,
    last_task_id: Arc<Mutex<String>>,
}
pub type Message = Vec<(String, Vec<(String, Vec<(String, Task)>)>)>;

impl RedisClient {
    pub fn new(redis_client: ClientRedis, sender: Sender<Task>, last_task_id: String) -> Self {
        Self {
            redis_client: Arc::new(redis_client),
            sender: Arc::new(sender),
            last_task_id: Arc::new(Mutex::new(last_task_id)),
        }
    }
    #[instrument(skip_all, fields(task_id), name = "send_tasks_to_executor")]
    async fn send(&self, message: Message) {
        let mut guard = self.last_task_id.lock().await;
        for message in message {
            for message in message.1 {
                *guard = message.0;
                for task in message.1 {
                    let span = Span::current();
                    span.record("task_id", task.1.id.to_string());
                    if let Err(err) = self.sender.send_async(task.1).await {
                        error!("{}", err)
                    } else {
                        debug!("ok")
                    }
                }
            }
        }
    }

    async fn get_last_task_id(&self) -> Option<String> {
        let guard = self.last_task_id.lock().await;
        if guard.is_empty() {
            return None;
        }
        Some(guard.to_string())
    }
    #[instrument(skip_all, name = "read_unprocessed_tasks")]
    async fn run_read_unprocessed_tasks(&self) -> RedisResult<()> {
        let opts = StreamReadOptions::default().count((i32::MAX) as usize);
        let last_task_id = self.get_last_task_id().await;
        if let Some(id) = last_task_id {
            info!("start read unprocessed_tasks from queue");
            let old_tasks = self.read_message::<Message>(&[&id], opts).await?;
            if old_tasks.len() < 1 {
                info!("unprocessed_tasks not found");
                return Ok(());
            }
            info!("send  unprocessed_tasks to TaskExecutor");
            self.send(old_tasks).await;
            info!("uprocessed_task sent to TaskExecutor");
        }
        Ok(())
    }
    #[instrument(skip_all, name = "recv_message")]
    async fn recv_messages(&self) {
        let opts = StreamReadOptions::default().block(0).count(1);
        if let Ok(message) = self.read_message::<Message>(&["$"], opts).await {
            info!("new task");
            self.send(message).await
        }
    }
    #[instrument(skip_all, name = "read_messages")]
    async fn read_message<T>(&self, ids: &[&str], opts: StreamReadOptions) -> RedisResult<T>
    where
        T: FromRedisValue,
    {
        let mut connection = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .inspect_err(|err| error!("get connection to Redis: {}", err))?;
        let task = connection
            .xread_options(&["inserts"], ids, &opts)
            .await
            .inspect_err(|err| error!("read tasks from message queue: {}", err))?;
        trace!("new message");
        Ok(task)
    }
}

#[async_trait]
impl RedisReceiver for RedisClient {
    async fn run(&self, token: CancellationToken) -> RedisResult<JoinHandle<()>> {
        self.run_read_unprocessed_tasks()
            .instrument(info_span!("RedisReceiver"))
            .await?;
        let receiver = self.clone();
        let task = spawn(async move {
            loop {
                select! {
                    _ = token.cancelled()=>{
                        break;
                    }
                    _ = receiver.recv_messages().instrument(info_span!("RedisReceiver"))=>{

                    }
                }
            }
        });
        Ok(task)
    }
    async fn stop(self) -> String {
        let sender = self.sender;
        drop(sender);
        let guard = self.last_task_id.lock().await;
        guard.clone()
    }
}
