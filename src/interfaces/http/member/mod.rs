use axum::Router;
use std::sync::Arc;
use axum::routing::get;
use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .route("/", get({
            || async { "Hello, World!" }
        }))
        .with_state(ctx.clone())
}
