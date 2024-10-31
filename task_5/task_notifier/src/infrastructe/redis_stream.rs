use std::{error::Error, sync::Arc, usize};

use axum::async_trait;
use deadpool_redis::{Connection, Pool, PoolError};
use log::trace;
use redis::{streams::StreamReadOptions, AsyncCommands, FromRedisValue};
use tokio::{select, spawn, sync::mpsc::UnboundedSender};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, error, info, info_span, instrument, Instrument, Span};
use crate::domain::{
    models::{LastTasks, LastTasksRedis, Message, Task, TypeNotification},
    redis_notifier::RedisNotifier,
};


enum TaskType{
    Completed,
    NonCompleted
}


#[derive(Clone)]
pub struct RedisStream {
    pool: Pool,
    cancellation_token: Arc<CancellationToken>,
    task_tracker: TaskTracker,
    completed_sender: Arc<UnboundedSender<Task>>,
    non_completed_sender: Arc<UnboundedSender<Task>>,
    last_tasks:Arc<LastTasksRedis>
}

impl RedisStream {
    pub fn new(
        pool: Pool,
        cancellation_token: CancellationToken,
        completed_sender: UnboundedSender<Task>,
        non_compeleted_sender: UnboundedSender<Task>,
        last_tasks:LastTasks
    ) -> Self {
        Self {
            pool,
            cancellation_token: Arc::new(cancellation_token),
            task_tracker: TaskTracker::new(),
            completed_sender: Arc::new(completed_sender),
            non_completed_sender: Arc::new(non_compeleted_sender),
            last_tasks:Arc::new(LastTasksRedis::from(last_tasks))
        }
    }
    async fn get_connection(&self) -> Result<Connection, PoolError> {
        self.pool.get().await
    }
    async fn get_message<T>(&self, keys: &[&str], ids: &[&str],opts:&StreamReadOptions) -> Result<T, PoolError>
    where
        T: FromRedisValue,
    {
        let mut connection = self.get_connection().await?;
        let messages = connection.xread_options(keys, ids, opts).await?;
        Ok(messages)
    }
    #[instrument(skip_all, name = "recv_non_completed_tasks")]
    async fn recv_non_completed_tasks(&self,opts:&StreamReadOptions)
    {

        if  let Ok(message) = self.get_message::<Message>(&["inserts"], &["$"],opts).await {
                self.send_notifications(message, TypeNotification::NonCompleted).await;
        }

    }
    #[instrument(skip_all, name = "recv_non_completed_tasks")]
    async fn get_completed_tasks(&self,opts:&StreamReadOptions)
    {
         if let Ok(message) =self.get_message::<Message>(&["updates"], &["$"],opts).await  {
            self.send_notifications(message, TypeNotification::Completed).await;
        }
    }

    async fn get_last_task_id(&self,task_type:&TaskType)->String{
          
                match task_type {
                    TaskType::Completed=>{
                            self.last_tasks.get_completed().await
                    }
                    _=>{
                        self.last_tasks.get_non_compeleted().await
                    }
                }
          
           
        }
       #[instrument(skip_all,name="send_notifications_TaskServiceNotifications")] 
    async fn send_notifications(&self,message: Message,notification_type:TypeNotification) {
        for message in message {
            for message in message.1 {
                match notification_type {
                    TypeNotification::Completed=>{
                        self.last_tasks.update_completed(message.0).await;
                    }
                    _=>{
                        self.last_tasks.update_non_completed(message.0).await;
                    }
                }
                for task in message.1 {
                    match notification_type {
                        TypeNotification::Completed=>{
                            if let Err(err)=self.completed_sender.send(task.1){
                                error!("{}",err)
                        }else { 
                                info!("notifications batch send to task_notifier_service")
                        }   
                        }
                        _=>{
                            if let Err(err)=self.non_completed_sender.send(task.1){
                                error!("{}",err)
                        }else { 
                                info!("batch send to task_notifier_service")
                        }   

                        }
                    }
                }
            }
        }
        }
            
    
    #[instrument(skip_all,name="read_unprocessed_notifications")]
    async fn read_unprocessed_notifications(&self,task_type:TaskType)->Result<(),PoolError>{
        let opts=StreamReadOptions::default().count((i32::MAX) as usize);
            match task_type {
                TaskType::Completed=>{
                    let task_id=self.last_tasks.get_completed().await;
                        if !task_id.is_empty() {
                            let message=self.get_message::<Message>(&["updates"], &[&task_id],&opts).await?;
                            println!("{:?}",message,);
                            if message.len()>0{
                                self.send_notifications(message, TypeNotification::Completed).await;
                            } else {
                                debug!("not found unprocessed notifications")
                            }
                        }else {
                            debug!("not found unprocessed notifications")
                            
                        }
                        Ok(()) 
                }
                _=>{
                    let task_id=self.last_tasks.get_non_compeleted().await;
                    if !task_id.is_empty(){
                        let message=self.get_message::<Message>(&["inserts"], &[&task_id],&opts).await?;
                        if message.len()>0{
                            self.send_notifications(message, TypeNotification::NonCompleted).await;
                            
                        } else {
                            debug!("not found unprocessed notifications")
                        }
                    }else {
                        debug!("not found unprocessed notifications")
                        
                    }
                    Ok(()) 

                }
            }
        }

    

    #[instrument(skip_all,name="read_notifications")]
    async fn read_notifications(&self)->Result<(),PoolError>{
        let consumer1 = self.clone();
        debug!("start read unprocessed_complete notifications");
        consumer1.read_unprocessed_notifications(TaskType::Completed).await?;
        debug!("end read unprocessed_complete notifications");
        let consumer2 = self.clone();
        debug!("start read unprocessed_non_complete notifications");
        consumer1.read_unprocessed_notifications(TaskType::NonCompleted).await?;
        debug!("end read unprocessed_non_complete notifications");
        let cancellation_token1 = self.cancellation_token.child_token();
        let cancellation_token2 = self.cancellation_token.child_token();
        let opts1 = StreamReadOptions::default().block(0).count(1);
        let opts2 = StreamReadOptions::default().block(0).count(1);
        let completed_tasks = spawn(async move {
            loop {
                select! {
                    _ = cancellation_token1.cancelled()=>{
                        trace!("RedisNotifier(completed_tasks) - handle stop signal");
                        break
                    }
                    _= consumer1.get_completed_tasks(&opts1).instrument(info_span!("RedisNotifier"))=>{
                    }
                }
            }
        });
        self.task_tracker.spawn(completed_tasks);

        let non_completed_tasks = spawn(async move {
            let span=Span::current();
            let _=span.entered();
            loop {
                select! {
                    _ = cancellation_token2.cancelled()=>{
                        trace!("RedisNotifier(non_completed_tasks) - handle stop signal");
                        break
                    }
                    _ = consumer2.recv_non_completed_tasks(&opts2).instrument(info_span!("RedisNotifier"))=>{

                    }
                }
            }
        });
        self.task_tracker.spawn(non_completed_tasks);
        Ok(())
    }
}


#[async_trait]
impl RedisNotifier for RedisStream {
    async fn run(&self) -> Result<(),Box<dyn Error>>{
        let consumer = self.clone();
        consumer.read_notifications().instrument(info_span!("RedisNotifier")).await?;
        Ok(())
    }
    async fn stop(&self)->LastTasks {
        self.task_tracker.close();
        self.completed_sender.closed().await;
        trace!("RedisNotifier - completed sender closed");
        self.non_completed_sender.closed().await;
        self.task_tracker.wait().await;
        trace!("RedisNotifier - task_tracker wait");
        trace!("RedisNotifier - non_ completed sender closed");
        self.pool.close();
        trace!("RedisNotifier - redis pool closed");
        let compelted_task_id=self.get_last_task_id(&TaskType::Completed).await;
        let non_completed_task_id=self.get_last_task_id(&TaskType::NonCompleted).await;
        LastTasks::new(compelted_task_id, non_completed_task_id)
    }
}

