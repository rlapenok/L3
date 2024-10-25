use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        create_task::create_task, get_task_info::get_task_info,
        update_task_status::update_task_status,
    },
    domain::task_service::TaskService
};

pub fn create_app<T>() -> Router<T>
where
    T: TaskService + Clone + Send + Sync + 'static,
{
    let router = Router::new()
        .route("/", post(create_task::<T>))
        .route("/:id", get(get_task_info::<T>))
        .route("/:id/complete", post(update_task_status::<T>));
    Router::new().nest("/tasks", router)
}

