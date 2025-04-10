pub mod handler;
use std::sync::Arc;
use axum::routing::get;
use axum::Router;

use crate::di::AppContext;

pub fn router(ctx: Arc<AppContext>) -> Router {

    Router::new()
        .route("/", get(handler::redirect_to_github_login_page))
        .route("/callback", get(handler::oauth2_sign_in))
        .with_state(ctx.clone())
}
