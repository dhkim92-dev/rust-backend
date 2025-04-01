use tracing::info;
use std::sync::{Arc};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::Json;
use shaku::{HasComponent};

use super::dto::{LoginRequest, LoginResponse};
use crate::di::AppContext;
use crate::common::wrapper::{ApiResponse, ReturnValue};
use crate::common::error::error_code::ErrorCode;
use crate::application::auth::usecases::LoginUseCase;

pub async fn login(
    State(ctx): State<Arc<AppContext>>,
    Json(req): Json<LoginRequest>
) -> Result<ReturnValue<LoginResponse>, ErrorCode> {
    info!("login request from {:?}", req.email);

    let login_usecase: &dyn LoginUseCase = ctx.resolve_ref();
    let result = login_usecase.login(
        crate::application::auth::usecases::LoginCommand{
            principal: req.email,
            credential: req.password,
        }).await?;

    let response = LoginResponse {
        typ: "Bearer".to_string(),
        access_token: result.access_token,
        refresh_token: result.refresh_token
    };
    Ok(ReturnValue::new(201, "로그인에 성공하였습니다.".to_string(), response))
}

