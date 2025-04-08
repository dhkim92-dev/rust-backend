use std::sync::Arc;
use axum::{middleware::from_fn, routing::put, Router};
use handler::upload_image;
use crate::{common::with_role_admin, di::AppContext};
pub mod handler;

pub fn router(ctx: Arc<AppContext>) -> Router {

    Router::new()
        .route("/images", put(upload_image))
        .route_layer(from_fn(with_role_admin))
        .with_state(ctx.clone())
}
