use crate::common::wrapper::ApiResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};
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

impl From<sea_orm::error::DbErr> for ErrorCode {
    fn from(err: sea_orm::error::DbErr) -> Self {
        ErrorCode::DBERROR
    }
}

impl ErrorCode {
    pub fn with_message(ecode: ErrorCode, message: &'static str) -> ErrorCode {
        ErrorCode {
            status: ecode.status,
            code: ecode.code,
            message: message,
        }
    }
    /**
     * 기본 에러 코드
     */

    pub const INTERNAL_SERVER_ERROR: ErrorCode = ErrorCode {
        status: 500,
        code: "GE-001",
        message: "서버 내부 오류입니다.",
    };

    pub const BAD_REQUEST: ErrorCode = ErrorCode {
        status: 400,
        code: "GE-002",
        message: "잘못된 요청입니다.",
    };

    pub const NOT_FOUND: ErrorCode = ErrorCode {
        status: 404,
        code: "GE-003",
        message: "요청한 리소스를 찾을 수 없습니다.",
    };

    pub const UNAUTHORIZED: ErrorCode = ErrorCode {
        status: 401,
        code: "GE-004",
        message: "인증되지 않은 사용자입니다.",
    };

    pub const FORBIDDEN: ErrorCode = ErrorCode {
        status: 403,
        code: "GE-005",
        message: "접근이 금지된 리소스입니다.",
    };

    pub const CONFLICT: ErrorCode = ErrorCode {
        status: 409,
        code: "GE-006",
        message: "이미 존재하는 리소스입니다.",
    };

    pub const DBERROR: ErrorCode = ErrorCode {
        status: 500,
        code: "GE-007",
        message: "DB 오류입니다.",
    };

    /**
     * 인증 / 인가 관련 에러 코드
     */
    pub const EMAIL_PASSWORD_MISMATCH: ErrorCode = ErrorCode {
        status: 401,
        code: "AE-001",
        message: "이메일 또는 비밀번호가 일치하지 않습니다.",
    };

    pub const JWT_BUILD_CLAIMS_EXCEPTION: ErrorCode = ErrorCode {
        status: 500,
        code: "AE-002",
        message: "JWT Claims 생성 중 오류가 발생했습니다.",
    };

    pub const INVALID_JWT_TOKEN: ErrorCode = ErrorCode {
        status: 401,
        code: "AE-003",
        message: "유효하지 않은 JWT 토큰입니다.",
    };

    pub const NOT_ENOUGH_PERMISSION: ErrorCode = ErrorCode {
        status: 403,
        code: "AE-004",
        message: "권한이 없습니다.",
    };

    /**
     * 멤버 에러 코드
     */
    pub const MEMBER_NOT_FOUND: ErrorCode = ErrorCode {
        status: 400,
        code: "ME-001",
        message: "존재하지 않는 회원입니다.",
    };

    pub const EMAIL_ALREADY_EXISTS: ErrorCode = ErrorCode {
        status: 400,
        code: "ME-002",
        message: "이미 존재하는 이메일입니다.",
    };
}
