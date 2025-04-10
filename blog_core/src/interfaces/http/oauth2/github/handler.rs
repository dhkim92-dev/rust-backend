use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use shaku::HasComponent;

use crate::application::auth::usecases::{LoginUseCase, OAuth2LoginCommand};
use crate::application::oauth2::github::GithubOAuth2UsecaseImpl;
use crate::application::oauth2::OAuth2Usecase;
use crate::common::error::error_code::ErrorCode;
use crate::common::{AppError, CookieMaker, ReturnValue};
use crate::config::OAuth2ConfigProvider;
use crate::di::AppContext;
use crate::interfaces::http::auth::dto::LoginResponse;

pub async fn redirect_to_github_login_page(
    cookie_jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    // let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let oauth2_provider: &dyn OAuth2ConfigProvider = ctx.resolve_ref();
    let cookie_maker: &dyn CookieMaker = ctx.resolve_ref();
    let github_oauth2_service = GithubOAuth2UsecaseImpl::new(
        oauth2_provider,
        cookie_maker,
    );
    let mode = query.get("mode")
        .ok_or_else(|| AppError::with_message(ErrorCode::BadRequest, "mode is required"))?
        .to_string();

    let (cookie_jar, redirect_uri) = github_oauth2_service.redirect_to_login_page(cookie_jar, mode);
    tracing::debug!("redirect_uri : {:?}", redirect_uri);
    tracing::debug!("cookie_jar : {:?}", cookie_jar);

    Ok((cookie_jar, Redirect::to(redirect_uri.to_string().as_str())).into_response())
}

pub async fn oauth2_sign_in(
    cookie_jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    // let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    tracing::debug!("/api/v1/oauth2/github/callback called");
    let oauth2_provider: &dyn OAuth2ConfigProvider = ctx.resolve_ref();
    let cookie_maker: &dyn CookieMaker = ctx.resolve_ref();
    let github_oauth2_service: &dyn OAuth2Usecase = &GithubOAuth2UsecaseImpl::new(
        oauth2_provider,
        cookie_maker,
    );

    let state = query.get("state").ok_or_else(|| {
        AppError::with_message(ErrorCode::Unauthorized, "state is required")
    })?;

    tracing::debug!("state : {:?}", state);
    
    let code = query.get("code").ok_or_else(|| {
        AppError::with_message(ErrorCode::Unauthorized, "code is required")
    })?;

    tracing::debug!("code : {:?}", code);

    let (cookie_jar, user_profile) = github_oauth2_service
        .get_userinfo(cookie_jar, 
            state.to_string(), 
            code.to_string())
        .await?;

    tracing::debug!("user_profile : {:?}", user_profile);

    let auth_service: &dyn LoginUseCase = ctx.resolve_ref();
    let login_command = OAuth2LoginCommand {
        provider: user_profile.provider,
        user_id: user_profile.user_id,
        email: user_profile.email,
        access_token: user_profile.access_token,
    };

    let login_result = auth_service.login_by_oauth2(login_command).await?;
    tracing::debug!("login_result : {:?}", login_result);

    let mut cookie = cookie_maker.create_cookie("refresh-token".to_owned(), login_result.refresh_token.clone());
    cookie.set_max_age(time::Duration::hours(24*15)); // 15
    let cookie_jar = cookie_jar.add(cookie);

    Ok((cookie_jar, ReturnValue {
        status: 201,
        data: LoginResponse {
            typ: "Bearer".to_string(),
            access_token: login_result.access_token,
            refresh_token: login_result.refresh_token,
        },
        message: "Github 로그인 성공".to_string(),
    }))
}
