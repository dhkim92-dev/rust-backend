pub mod handler;
pub mod vo;
pub mod dto;

use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use handler::login;
use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", post(login))
        .with_state(ctx.clone())
}
