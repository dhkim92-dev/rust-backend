
use axum::{http::StatusCode, response::IntoResponse, Json};
use error_code_macro::ErrorCode;

use crate::common::wrapper::ApiResponse;

#[derive(ErrorCode)]
pub struct AuthError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
}

impl AuthError {

    pub const EmailPasswordMismatch: Self = Self {
        status: StatusCode::UNAUTHORIZED,
        code: "AE-001",
        message: "이메일 또는 비밀번호가 잘못되었습니다.",
    };
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = self.error();
        let response: ApiResponse<String> = ApiResponse {
            timestamp: chrono::Utc::now().naive_utc(),
            status: status.as_u16() as i32,
            data: None,
            code: Some(code.to_string()),
            message: message.to_string(),
        };

        (status, Json(response)).into_response()
    }
}
