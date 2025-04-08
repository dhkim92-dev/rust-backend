use std::sync::Arc;
use axum::extract::State;
use axum::response::{Redirect, Response};
use axum_extra::extract::CookieJar;

use crate::common::redirect_uri_builder::RedirectUriBuilder;
use crate::common::AppError;
use crate::config::{ConfigProvider, OAuth2ConfigProvider};
use crate::di::AppContext;


pub async fn redirect_to_github_login_page(
    cookie_jar: CookieJar,
    State(ctx): State<Arc<AppContext>>
) -> Redirect {
    // let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    // let oauth2_provider: &dyn OAuth2ConfigProvider = ctx.resolve_ref();
    // let redirect_uri_builder = RedirectUriBuilder::new();

    Redirect::to("https://www.dohoon-kim.kr")
}
async fn try_to_exchange_access_token(
    State(ctx): State<Arc<AppContext>>,
) {

}

async fn try_to_get_user_info(
    State(ctx): State<Arc<AppContext>>
) {

}
