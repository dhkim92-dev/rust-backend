use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::common::wrapper::ApiResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorCode {
    pub status: u16,
    pub code: &'static str,
    pub message: &'static str,
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let response = ApiResponse {
            timestamp: chrono::Utc::now().naive_utc(),
            status: self.status,
            data: None as Option<String>,
            code: Some(self.code.to_string()),
            message: self.message.to_string(),
        };
        (status, Json(response)).into_response()
    }
}

impl ErrorCode {
    pub fn with_message(ecode: ErrorCode, message: &'static str) -> ErrorCode {
        ErrorCode {
            status: ecode.status,
            code: ecode.code,
            message: message
        }
    }

    pub const EMAIL_PASSWORD_MISMATCH: ErrorCode = ErrorCode {
        status: 401, 
        code: "AE-001", 
        message: "이메일 또는 비밀번호가 일치하지 않습니다."
    };
    
    pub const MEMBER_NOT_FOUND: ErrorCode = ErrorCode {
        status: 400, 
        code: "ME-001",
        message: "존재하지 않는 회원입니다."
    };

    pub const INVALID_JWT_TOKEN: ErrorCode = ErrorCode {
        status: 401, 
        code: "AE-002", 
        message: "유효하지 않은 JWT 토큰입니다."
    };

    pub const NOT_ENOUGH_PERMISSION: ErrorCode = ErrorCode {
        status: 403, 
        code: "AE-003", 
        message: "권한이 없습니다."
    };
}
