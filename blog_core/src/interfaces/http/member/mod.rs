mod handler;
use crate::{common::middleware::security::with_role_member, di::AppContext};
use axum::routing::{delete, post, put};
use axum::{middleware::from_fn, Router};
use handler::{create_new_member, delete_member, update_member_info};
use std::sync::Arc;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", post(create_new_member))
        .route(
            "/{id}",
            put(update_member_info).route_layer(from_fn(with_role_member)),
        )
        .route(
            "/{id}",
            delete(delete_member).route_layer(from_fn(with_role_member)),
        )
        .with_state(ctx.clone())
}
