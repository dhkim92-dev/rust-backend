pub mod auth;
pub mod board;
pub mod member;
pub mod test;

use crate::{common::middleware::security::jwt_authentication_filter, di::AppContext};
use axum::{
    middleware::{from_fn_with_state},
    Router,
};
use std::sync::Arc;

pub fn create_routers(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .nest("/api/v1/tests", test::router(ctx.clone()))
        .nest("/api/v1/auth", auth::router(ctx.clone()))
        .nest("/api/v1/members", member::router(ctx.clone()))
        .nest("/api/v1/boards", board::router(ctx.clone()))
        .layer(from_fn_with_state(ctx.clone(), jwt_authentication_filter))
}
