use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use shaku::HasComponent;
use std::sync::Arc;
use tracing::{debug, info};

use super::dto::{JwtReissueResponse, LoginRequest, LoginResponse};
use crate::application::auth::usecases::LoginUseCase;
use crate::application::auth::JwtUseCase;
use crate::common::error::error_code::ErrorCode;
use crate::common::wrapper::ReturnValue;
use crate::common::CookieBuilder;
use crate::config::ConfigProvider;
use crate::di::AppContext;

pub async fn reissue_jwt(
    jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let jwt_usecase: &dyn JwtUseCase = ctx.resolve_ref();
    // let cookie_builder: &dyn CookieBuilder = ctx.resolve_ref();

    let rt_cookie = match jar.get("refresh-token") {
        Some(cookie) => cookie,
        None => {
            tracing::error!("refresh token이 존재하지 않습니다.");
            return Err((jar, ErrorCode::InvalidJwtToken));
        }
    };
    let mut remove_cookie = Cookie::new("refresh-token", "");
    remove_cookie.set_path("/");
    let token_string = rt_cookie.value().to_string();

    match jwt_usecase.refresh_jwt(token_string).await {
        Ok(jwt_result) => {
            debug!("jwt result : {:?}", jwt_result);
            let payload = JwtReissueResponse {
                access_token: jwt_result.access_token,
            };
            Ok((
                jar,
                ReturnValue::new(
                    201,
                    "access token 재발급에 성공하였습니다.".to_string(),
                    payload,
                ),
            ))
        }
        Err(err) => {
            tracing::error!("access token 재발급에 실패하였습니다. refresh token이 만료되었거나 변조되었습습니다.");
            let jar = jar.remove(remove_cookie);
            Err((jar, ErrorCode::InvalidJwtToken))
        }
    }
}

pub async fn login(
    mut jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl axum::response::IntoResponse, ErrorCode> {
    info!("login request from {:?}", req.email);
    let login_usecase: &dyn LoginUseCase = ctx.resolve_ref();
    let cookie_builder: &dyn CookieBuilder = ctx.resolve_ref();
    let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let cfg = config_provider.get();

    let result = login_usecase
        .login(crate::application::auth::usecases::LoginCommand {
            principal: req.email,
            credential: req.password,
        })
        .await?;

    let payload = LoginResponse {
        typ: "Bearer".to_string(),
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    };

    let mut refresh_token_cookie = Cookie::new("refresh-token", payload.refresh_token.clone());
    refresh_token_cookie.set_path("/");
    refresh_token_cookie.set_http_only(cfg.is_production());
    refresh_token_cookie.set_secure(cfg.is_production());
    refresh_token_cookie.set_max_age(time::Duration::milliseconds(
        cfg.jwt_refresh_token_expire as i64,
    ));

    jar = jar.add(refresh_token_cookie.clone());
    Ok((
        jar,
        ReturnValue::new(201, "로그인에 성공하였습니다.".to_string(), payload),
    ))
}
