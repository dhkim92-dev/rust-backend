pub mod handler;
use std::sync::Arc;
use axum::routing::get;
use axum::Router;
use rand::Rng;
use rand::distributions::Alphanumeric;

use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> Router {

    Router::new()
        .route("/", get(handler::redirect_to_github_login_page))
        .route("/callback", get(handler::try_to_exchange_access_token))
        .with_state(ctx.clone())
}
