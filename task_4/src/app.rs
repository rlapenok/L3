use axum::{
    routing::{post, put},
    Router,
};

use crate::{
    domain::{product_service::ProductService, user_service::UserService},
    handlers::{
        products_handlers::{create_product::create_product, delete_product::delete_product, update_product::update_product}, user_handlers::{
            create_user::create_user, delete_user::delete_user, update_user::update_user,
        }
    },
};

pub fn create_app<T>() -> Router<T>
where
    T: UserService+ProductService + Clone + Send + Sync + 'static,
{
    let product_router = Router::new().route("/", post(create_product::<T>)).route("/:id", put(update_product::<T>)).route("/:id", post(delete_product::<T>));
    let user_router = Router::new()
        .route("/", post(create_user::<T>))
        .route("/:id", put(update_user::<T>))
        .route("/:id", post(delete_user::<T>));
    let router = Router::new()
        .nest("/products", product_router)
        .nest("/users", user_router);
    router
}
