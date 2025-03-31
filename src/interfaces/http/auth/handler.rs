use tracing::info;
use std::sync::{Arc};
use axum::response::IntoResponse;
use axum::extract::State;
use axum::Json;
use shaku::{HasComponent};

use super::dto::{LoginRequest, LoginResponse};
// use crate::config::AppContext;
use crate::di::AppContext;
use crate::common::wrapper::ApiResponse;
use crate::common::error::error_code::ErrorCode;
use crate::application::auth::usecases::LoginUseCase;

pub async fn login(
    State(ctx): State<Arc<AppContext>>,
    Json(req): Json<LoginRequest>
) -> Result<impl IntoResponse, ErrorCode> {
    println!("Login request called");
    let login_usecase: &dyn LoginUseCase = ctx.resolve_ref();
    let result = login_usecase.login(
        crate::application::auth::usecases::LoginCommand{
            principal: req.email,
            credential: req.password,
        }).await?;

    let response = LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    };

    Ok(ApiResponse::<LoginResponse>::new(201, "로그인 성공".to_string(), Some(response)))
}

