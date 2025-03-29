use axum::{http::StatusCode, response::IntoResponse, Json};
use error_code_macro::ErrorCode;

use crate::common::wrapper::ApiResponse;

#[derive(ErrorCode)]
pub struct MemberError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
}

impl MemberError {

    pub const MemberNotExist: Self = Self {
        status: StatusCode::NOT_FOUND,
        code: "AE-001",
        message: "이메일 또는 비밀번호가 잘못되었습니다.",
    };

    pub const EmailAlreadyExist: Self = Self {
        status: StatusCode::BAD_REQUEST,
        code: "AE-002",
        message: "이미 존재하는 이메일입니다.",
    };
}

impl IntoResponse for MemberError {
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
