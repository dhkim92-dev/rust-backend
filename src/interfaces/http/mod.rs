pub mod auth;
pub mod member;
use std::sync::{Arc};
use axum::{
    Router,
};

use crate::config::AppContext;

pub fn create_router(ctx: Arc<AppContext>) -> Router {

    Router::new()
        .nest("/api/v1/auth", auth::handler::router(ctx.clone()))

}

