use axum::async_trait;

use super::
    models::{Task, TypeNotification}
;

#[async_trait]
pub trait NotificationService {
    async fn get_notifications(
        &self,
        type_notifications: TypeNotification,
    ) -> Vec<Task>;
}
