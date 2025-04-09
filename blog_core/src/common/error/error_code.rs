use crate::common::wrapper::ApiResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    InternalServerError,
    BadRequest,
    NotFound,
    Unauthorized,
    Forbidden,
    Conflict,
    DbError,
    NotImplemented,
    InvalidInput,
    // 검증 에러
    ValidationError,

    // 인증 관련 에러
    EmailPasswordMismatch,
    JwtBuildClaimsException,
    InvalidJwtToken,

    //인가 관련 에러
    NotEnoughPermission,

    // 멤버 에러 코드
    MemberNotFound,
    EmailAlreadyExists,

    // OAuth2 관련 에러 
    FailedToGetAuthorizationGrantCode,
    FailedToGetAccessToken,
    FailedToDeserializeAccessToken,
    FailedToGetUserProfile,
    FailedToDeserializeUserProfile,
    StateMismatch,
}

impl ErrorCode {
    pub fn cast(self) -> (StatusCode, &'static str, &'static str) {
        match self {
            // 일반 에러
            Self::BadRequest => (StatusCode::BAD_REQUEST, "GE-001", "잘못된 요청입니다."),

            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "GE-002",
                "인증되지 않은 사용자입니다.",
            ),

            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                "GE-003",
                "접근이 금지된 리소스입니다.",
            ),

            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "GE-004",
                "요청한 리소스를 찾을 수 없습니다.",
            ),
            
            Self::Conflict => (
                StatusCode::CONFLICT,
                "GE-005",
                "이미 존재하는 리소스입니다.",
            ),

            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GE-006",
                "서버 내부 오류입니다.",
            ),

            Self::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "GE-007",
                "구현되지 않은 기능입니다.",
            ),

            Self::DbError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GE-008",
                "DB 오류입니다.",
            ),

            // 검증 에러
            Self::ValidationError => (StatusCode::BAD_REQUEST, "GE-008", "검증 오류입니다."),

            Self::InvalidInput => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GE-009",
                "잘못된 입력입니다.",
            ),

            // 인증 관련 에러
            Self::EmailPasswordMismatch => (
                StatusCode::UNAUTHORIZED,
                "AE-001",
                "이메일 또는 비밀번호가 일치하지 않습니다.",
            ),
            Self::JwtBuildClaimsException => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "AE-002",
                "JWT Claims 생성 중 오류가 발생했습니다.",
            ),
            Self::InvalidJwtToken => (
                StatusCode::UNAUTHORIZED,
                "AE-003",
                "유효하지 않은 JWT 토큰입니다.",
            ),

            // 인가 관련 에러
            Self::NotEnoughPermission => (StatusCode::FORBIDDEN, "AE-004", "권한이 없습니다."),

            // 멤버 관련 에러
            Self::MemberNotFound => (
                StatusCode::BAD_REQUEST,
                "ME-001",
                "존재하지 않는 회원입니다.",
            ),
            Self::EmailAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "ME-002",
                "이미 존재하는 이메일입니다.",
            ),

            // OAuth2 관련 에러
            Self::FailedToGetAuthorizationGrantCode => (
                StatusCode::UNAUTHORIZED,
                "OE-001",
                "Authorization Grant Code를 가져오는 데 실패했습니다.",
            ),
            Self::FailedToGetAccessToken => (
                StatusCode::UNAUTHORIZED,
                "OE-002",
                "Access Token을 가져오는 데 실패했습니다.",
            ),
            Self::FailedToDeserializeAccessToken => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "OE-003",
                "Access Token을 역직렬화하는 데 실패했습니다.",
            ),
            Self::FailedToGetUserProfile => (
                StatusCode::UNAUTHORIZED,
                "OE-004",
                "사용자 프로필을 가져오는 데 실패했습니다.",
            ),
            Self::FailedToDeserializeUserProfile => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "OE-005",
                "사용자 프로필을 역직렬화하는 데 실패했습니다.",
            ),
            Self::StateMismatch => (
                StatusCode::UNAUTHORIZED,
                "OE-006",
                "State 값이 일치하지 않습니다.",
            ),
        }
    }

    pub fn with_message(code: ErrorCode, message: &'static str) -> ApiResponse<String> {
        let (status, code, _message) = code.cast();
        ApiResponse {
            timestamp: chrono::Utc::now().naive_utc(),
            status: status.as_u16(),
            data: None as Option<String>,
            code: Some(code.to_owned()),
            message: message.to_owned(),
        }
    }
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> axum::response::Response {
        let (_status, code, message) = self.cast();
        let response = ApiResponse {
            timestamp: chrono::Utc::now().naive_utc(),
            status: _status.as_u16(),
            data: None as Option<String>,
            code: Some(code.to_owned()),
            message: message.to_owned(),
        };
        (_status, Json(response)).into_response()
    }
}

impl From<sea_orm::error::DbErr> for ErrorCode {
    fn from(err: sea_orm::error::DbErr) -> Self {
        tracing::error!("DB Error: {}", err);
        ErrorCode::DbError
    }
}
