pub mod handler;
use axum::middleware::from_fn;
use axum::Router;
pub use handler::*;

use axum::routing::{post, put, get, delete};
use std::sync::Arc;
use crate::common::with_role_admin;
use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> axum::Router {

    let post_command_router = axum::Router::new()
        .route("/", post(create_post))
        .route("/{id}", put(update_post))
        .route("/{id}", delete(delete_post))
        .layer(from_fn(with_role_admin))
        .with_state(ctx.clone());

    let post_query_router = axum::Router::new()
        .route("/", get(get_posts))
        .with_state(ctx.clone());

    Router::new()
        .merge(post_command_router)
        .merge(post_query_router)
}
