use axum::extract::State;
use axum::{Extension, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use shaku::HasComponent;
use std::sync::Arc;
use tracing::info;

use super::dto::{JwtReissueResponse, LoginRequest, LoginResponse};
use crate::application::auth::usecases::{LoginUseCase};
use crate::application::auth::JwtUseCase;
use crate::common::error::error_code::ErrorCode;
use crate::common::wrapper::ReturnValue;
use crate::config::{ConfigProvider};
use crate::di::AppContext;

fn extract_refresh_token_from_cookie(jar: &CookieJar) -> Option<String> {
    jar.get("refresh-token")
        .map(|cookie| cookie.value().to_string())
}

pub async fn reissue_jwt(
    Extension(jar): Extension<CookieJar>,
    State(ctx): State<Arc<AppContext>>,
) -> Result<ReturnValue<JwtReissueResponse>, ErrorCode> {
    let jwt_usecase: &dyn JwtUseCase = ctx.resolve_ref();
    let refresh_token = extract_refresh_token_from_cookie(&jar);

    if refresh_token.is_none() {
        return Err(ErrorCode::INVALID_JWT_TOKEN);
    }

    let token_string = refresh_token.as_ref().unwrap();
    let refresh_token_cookie = Cookie::new("refresh-token", token_string.to_string());

    let result = jwt_usecase
        .refresh_jwt(token_string.to_string())
        .await
        .map_err(|x| {
            jar.remove(Cookie::new("refresh-token", ""));
            tracing::error!("access token 재발급에 실패하였습니다.");
            return ErrorCode::INVALID_JWT_TOKEN;
        });

    match result {
        Ok(jwt_result) => Ok(ReturnValue::new(
            201,
            "JWT 재발급에 성공하였습니다.".to_string(),
            JwtReissueResponse {
                access_token: jwt_result.access_token.to_string(),
            },
        )),
        Err(err) => {
            return Err(ErrorCode::INVALID_JWT_TOKEN);
        }
    }
}

pub async fn login(
    jar: CookieJar,
    State(ctx): State<Arc<AppContext>>,
    Json(req): Json<LoginRequest>,
) -> Result<ReturnValue<LoginResponse>, ErrorCode> {
    info!("login request from {:?}", req.email);
    let login_usecase: &dyn LoginUseCase = ctx.resolve_ref();
    let result = login_usecase
        .login(crate::application::auth::usecases::LoginCommand {
            principal: req.email,
            credential: req.password,
        })
        .await?;

    let response = LoginResponse {
        typ: "Bearer".to_string(),
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    };

    let cfg_provider: &dyn ConfigProvider = ctx.resolve_ref();

    let mut cookie = Cookie::new("refresh-token", response.refresh_token.to_string());
    cookie.set_domain(cfg_provider.get().server_host.clone());
    cookie.set_path("/");
    // cookie.set_http_only(true);
    // cookie.set_secure(true);

    jar.add(cookie);

    Ok(ReturnValue::new(
        201,
        "로그인에 성공하였습니다.".to_string(),
        response,
    ))
}
