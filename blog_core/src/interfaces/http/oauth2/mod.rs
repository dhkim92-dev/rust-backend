pub mod github;

use std::sync::Arc;
use axum::Router;
use crate::di::AppContext;


pub fn router(ctx: Arc<AppContext>) -> Router {
    Router::new()
        .nest("/github", github::router(ctx))
}
