use axum::extract::{Query, State};
use tracing::{info, instrument};

use crate::domain:: task_notifier_service::NotificationService;

use super::{reponses::Notification, requests::Params};

#[instrument(skip_all, name = "get_notifications_handler")]
pub async fn get_notifications<T: NotificationService>(
    State(state): State<T>,
    Query(params): Query<Params>,
) -> Notification {
    let tasks = state.get_notifications(params.notification_type).await;
    info!("Ok");
    Notification::new(tasks)
}
