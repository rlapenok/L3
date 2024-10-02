use axum::{
    middleware::from_fn_with_state, routing::{delete, get, post, IntoMakeService}, Router
};

use crate::{
    api::{create_post::create_post, delete_post::delete_post, get_post::get_post, like_post::like_post, login::login, middleware::authorization, register::register},
    infrastructe::server_state::ServerState,
};

pub fn create_app(state:ServerState) -> IntoMakeService<Router> {
    let router = Router::new()
        .route("/", post(create_post))
        .route("/:post_id", get(get_post))
        .route("/:post_id", delete(delete_post))
        .route("/:post_id/likes", post(like_post))
        .route_layer(from_fn_with_state(state.clone(),authorization));
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .nest("/posts", router)
        .with_state(state).into_make_service()
}
