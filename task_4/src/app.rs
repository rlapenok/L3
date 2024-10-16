use axum::{routing::post, Router};

use crate::handlers::create_product::create_product;

pub fn create_app()->Router{
    let product_router=Router::new().
    route("/", post(create_product));

    let router=Router::new().nest("/product", product_router);
    router
}