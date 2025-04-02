pub mod dto;
pub mod handler;
pub mod vo;

use crate::di::AppContext;
use axum::routing::post;
use axum::Router;
use handler::login;
use std::sync::Arc;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", post(login))
        .route("/jwt/reissue", post(handler::reissue_jwt))
        .with_state(ctx.clone())
}
