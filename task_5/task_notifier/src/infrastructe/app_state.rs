use std::sync::Arc;

use axum::async_trait;
use log::{debug, info, trace};
use tracing::debug as tracing_debug;
use tokio::{signal::ctrl_c, sync::{mpsc::UnboundedReceiver, Mutex}};
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use crate::domain::{
    last_notifier_tasks::LastNotifierTasks,
    models::{ Task, TypeNotification},
    redis_notifier::RedisNotifier,
    task_notifier_service::NotificationService,
};

#[derive(Clone)]
pub struct AppState<R, L>
where
    R: RedisNotifier + Clone + Send + Sync,
    L: LastNotifierTasks + Clone + Send + Sync,
{
    redis_notifier: R,
    last_notifier_tasks: L,
    completed_receiver: Arc<Mutex<UnboundedReceiver<Task>>>,
    non_completed_receiver: Arc<Mutex<UnboundedReceiver<Task>>>,
    cancellation_token: Arc<CancellationToken>,
}

impl<R, L> AppState<R, L>
where
    R: RedisNotifier + Clone + Send + Sync,
    L: LastNotifierTasks + Clone + Send + Sync,
{
    pub fn new(
        redis_notifier: R,
        last_notifier_tasks: L,
        non_completed_receiver: UnboundedReceiver<Task>,
        completed_receiver: UnboundedReceiver<Task>,
        canecllation_token: CancellationToken,
    ) -> Self {
        Self {
            redis_notifier,
            last_notifier_tasks,
            non_completed_receiver: Arc::new(Mutex::new(non_completed_receiver)),
            completed_receiver: Arc::new(Mutex::new(completed_receiver)),
            cancellation_token: Arc::new(canecllation_token),
        }
    }
}

#[async_trait]
impl<R, L> NotificationService for AppState<R, L>
where
    R: RedisNotifier + Clone + Send + Sync,
    L: LastNotifierTasks + Clone + Send + Sync,
{
    #[instrument(skip_all, name = "NotificationsService::get_notifications")]
    async fn get_notifications(
        &self,
        type_notifications: TypeNotification,
    ) -> Vec<Task> {
        let mut buffer=Vec::new();
        match type_notifications {
            TypeNotification::Completed=>{
                let mut guard=self.completed_receiver.lock().await;
                while let Ok(notifications) =guard.try_recv()  {
                        buffer.push(notifications);
                }
            }
            _=>{
                let mut guard=self.non_completed_receiver.lock().await;
                while let Ok(notifications) =guard.try_recv()  {
                    buffer.push(notifications);
            }

            }

        };
        tracing_debug!("notifications received");
     buffer
    }
}

pub async fn gracefull_shutdown<R, L>(state: AppState<R,L>)
where
    R: RedisNotifier + Clone + Send + Sync,
    L: LastNotifierTasks + Clone + Send + Sync,
{
    ctrl_c().await.expect("failed to install Ctrl+C handler");
    info!("Start gracefull shutdown server");
    state.cancellation_token.cancel();
    let mut guard1=state.completed_receiver.lock().await;
    let mut guard2=state.non_completed_receiver.lock().await;
    guard1.close();
    guard2.close();
    trace!("TaskNotifierService - tasks receiver closed");
    let last_task=state.redis_notifier.stop().await;
    debug!("RedisNotifier - STOP");
    state.last_notifier_tasks.save_last_tasks(last_task).await.expect("save last");
    info!("Server gracefully shutdown");
}