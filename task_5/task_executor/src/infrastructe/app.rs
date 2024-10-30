use std::error::Error;

use log::{error, info};
use tokio::signal::ctrl_c;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    domain::{redis_receiver::RedisReceiver, task_executor::TaskExecutor},
    infrastructe::last_task_manager::LastTaskManager,
};

pub struct App<R, T, E>
where
    R: RedisReceiver,
    T: LastTaskManager,
    E: TaskExecutor,
{
    redis_receiver: R,
    last_task_manager: T,
    task_tracker: TaskTracker,
    executor: E,
}

impl<R, T, E> App<R, T, E>
where
    R: RedisReceiver,
    T: LastTaskManager,
    E: TaskExecutor,
{
    pub fn new(redis_receiver: R, last_task_manager: T, executor: E) -> Self {
        Self {
            redis_receiver,
            last_task_manager,
            task_tracker: TaskTracker::new(),
            executor,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        //create cancellation token
        let cancellation_token = CancellationToken::new();
        //task for redis receiver
        let task = self
            .redis_receiver
            .run(cancellation_token.child_token())
            .await?;
        self.task_tracker.spawn(task);
        //run workers (executor)
        self.executor.run(cancellation_token.child_token());
        //close task tracker
        self.task_tracker.close();
        //wait ctrl_c signal
        ctrl_c().await.expect("msg");
        info!("Start gracefull shutdown task executor");
        //cancel_token
        cancellation_token.cancel();
        //stop readis receiver
        let last_task_id = self.redis_receiver.stop().await;
        info!("RedisReceiver - STOP");
        //wait background tasks
        self.task_tracker.wait().await;
        //stop workers(executor)
        self.executor.stop().await;
        info!("TaskExecutor (internal:workers) - STOP");
        self.last_task_manager
            .save_last_task_id(last_task_id)
            .await.inspect_err(|err|{
                error!("Error while save last task_id in file: {}",err)
            })?;
        info!("LastTaskManager - STOP");
        info!("Executor gracefully shutdown");
        Ok(())
    }
}
