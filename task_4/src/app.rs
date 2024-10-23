use std::time::Instant;

use axum::{
    body::Body,
    extract::Request,
    middleware::{from_fn, Next},
    response::Response,
    routing::{delete, post, put},
    Router,
};
use log::info;

use crate::{
    domain::{product_service::ProductService, user_service::UserService},
    handlers::{
        products_handlers::{
            create_product::create_product, delete_product::delete_product,
            update_product::update_product,
        },
        user_handlers::{
            create_user::create_user, delete_user::delete_user, update_user::update_user,
        },
    },
};

pub async fn logger_middleware(req: Request<Body>, next: Next) -> Response {
    let now = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let resp = next.run(req).await;
    let end = now.elapsed().as_secs_f32() * 1000.0;
    info!("   METHOD: {}   PATH: {}   elapsed:{}ms", method, path, end);
    resp
}

pub fn create_app<T>() -> Router<T>
where
    T: UserService + ProductService + Clone + Send + Sync + 'static,
{
    let product_router = Router::new()
        .route("/", post(create_product::<T>))
        .route("/:id", put(update_product::<T>))
        .route("/:id", delete(delete_product::<T>));
    let user_router = Router::new()
        .route("/", post(create_user::<T>))
        .route("/:id", put(update_user::<T>))
        .route("/:id", delete(delete_user::<T>));
    Router::new()
        .nest("/products", product_router)
        .nest("/users", user_router)
        .layer(from_fn(logger_middleware))
}
