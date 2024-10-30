use axum::{
    body::Body,
    extract::Request,
    routing::{get, post},
    Router,
};

use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

use crate::{
    api::{
        create_task::create_task, get_task_info::get_task_info,
        update_task_status::update_task_status,
    },
    domain::task_service::TaskService,
};

pub fn create_app<T>() -> Router<T>
where
    T: TaskService + Clone + Send + Sync + 'static,
{
    let router = Router::new()
        .route("/", post(create_task::<T>))
        .route("/:id", get(get_task_info::<T>))
        .route("/:id/complete", post(update_task_status::<T>));
    Router::new()
        .nest("/tasks", router).layer(TraceLayer::new_for_http().make_span_with(make_span))
}

fn make_span(req: &Request<Body>) -> Span {
    let headers = req.headers();
    let path = req.uri().path();
    let method=req.method();
    info_span!("new_request",method=%method,path=path,headers=?headers)
}
