mod handler;
use axum::Router;
use std::sync::Arc;
use axum::routing::{post, put, get, delete};
use handler::{create_new_member, update_member_info, delete_member};
use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", post(create_new_member))
        .route("/{id}", put(update_member_info))
        .route("/{id}", delete(delete_member))
        .with_state(ctx.clone())
}

