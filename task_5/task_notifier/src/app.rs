use axum::{body::Body, extract::Request, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

use crate::{
    api::get_notifications::get_notifications, domain::task_notifier_service::NotificationService,
};

pub fn create_app<T>() -> Router<T>
where
    T: NotificationService + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/notifications", get(get_notifications::<T>))
        .layer(TraceLayer::new_for_http().make_span_with(make_span))
}

fn make_span(req: &Request<Body>) -> Span {
    let headers = req.headers();
    let path = req.uri().path();
    let method = req.method();
    info_span!("new_request",method=%method,path=path,headers=?headers)
}
