mod auth;
mod member;

use std::sync::Arc;
use axum::Router;
use crate::di::AppContext;

pub fn create_routers(ctx: Arc<AppContext>)-> Router {

    Router::new()
        .nest("/api/v1/auth", auth::router(ctx.clone()))
        .nest("/api/v1/members", member::router(ctx.clone()))
}
