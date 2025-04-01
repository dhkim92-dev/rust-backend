mod handler;
use axum::{middleware::{self, from_fn}, Router};
use std::{ sync::Arc};
use axum::routing::{post, put, get, delete};
use handler::{create_new_member, update_member_info, delete_member};
use crate::{common::middleware::security::with_role_member, di::AppContext};

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", post(create_new_member))
        .route("/{id}", put(update_member_info)
            .route_layer(from_fn(with_role_member)))
        .route("/{id}", delete(delete_member)
            .route_layer(from_fn(with_role_member)))
        .with_state(ctx.clone())
}

