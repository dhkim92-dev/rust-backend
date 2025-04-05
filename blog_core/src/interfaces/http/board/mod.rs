pub mod handler;

use axum::{middleware::from_fn, routing::post};
use handler::*;
use std::sync::Arc;

use crate::{common::with_role_admin, di::AppContext};

pub fn router(ctx: Arc<AppContext>) -> axum::Router {
    axum::Router::new()
        .route(
            "",
            post(create_board).route_layer(from_fn(with_role_admin)),
        )
        .with_state(ctx.clone())
}
