use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use shaku::HasComponent;

// use crate::common::error::error_code::ErrorCode;
use crate::common::redirect_uri_builder::OAuth2RedirectURI;
use crate::common::{AppError, ReturnValue};
use crate::config::{ConfigProvider, OAuth2ConfigProvider};
use crate::di::AppContext;
use crate::interfaces::http::auth::dto::LoginResponse;


pub async fn redirect_to_github_login_page(
    mut cookie_jar: CookieJar,
    State(ctx): State<Arc<AppContext>>
) -> Result<impl IntoResponse, AppError> {
    let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let oauth2_provider: &dyn OAuth2ConfigProvider = ctx.resolve_ref();
    let cfg = config_provider.get();
    let state: String = super::generate_rand(32);
// /* 
    let redirect_uri = OAuth2RedirectURI::builder()
        .base_url(oauth2_provider.github_login_url().as_str())
        .redirect_uri(oauth2_provider.github_code_redirect_uri().as_str())
        .client_id(oauth2_provider.github_client_id().as_str())
        .response_type("code")
        .scope(oauth2_provider.github_scopes().as_str())
        .state(state.as_str())
        .build()
        .map_err(|e| {
            panic!("Error building redirect URI");
        })
        .unwrap();

    let mut state_cookie = Cookie::new("oauth2-state", state.clone());
    state_cookie.set_path("/");
    state_cookie.set_http_only(cfg.is_production());
    state_cookie.set_secure(cfg.is_production());
    state_cookie.set_same_site(axum_extra::extract::cookie::SameSite::Lax);
    state_cookie.set_max_age(time::Duration::seconds(180));
    cookie_jar = cookie_jar.add(state_cookie);

    //Ok((cookie_jar, Redirect::to("http://localhost:3000/")).into_response())

    Ok((cookie_jar, Redirect::to(redirect_uri.to_string().as_str())).into_response())
}

pub async fn try_to_exchange_access_token(
    mut cookie_jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
    Query(queries): Query< HashMap<String, String> >,
) -> Result<ReturnValue<LoginResponse>, AppError> {
    let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let oauth2_provider: &dyn OAuth2ConfigProvider = ctx.resolve_ref();
    let cfg = config_provider.get();

    let mut state_cookie = cookie_jar
        .get("oauth2-state")
        .unwrap()
        .clone();

    tracing::debug!("queries : {:?}", queries);
    tracing::debug!("state : {:?}", state_cookie.value());

    Ok(ReturnValue { 
        status: 201, 
        message: "Github Login 성공".to_string(), 
        data: LoginResponse {
            typ: "Bearer".to_string(),
            access_token: "hello world!".to_string(),
            refresh_token: "hello world!".to_string(),
        }
    })
}

async fn try_to_get_user_info(
    State(ctx): State<Arc<AppContext>>
) {

}
