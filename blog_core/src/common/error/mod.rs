use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::DbErr;

use self::error_code::ErrorCode;

use super::ApiResponse;


pub mod error_code;

pub struct AppError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn with_message(code: ErrorCode, msg: &'static str) -> Self {
        let (status, code, message) = code.cast();
        AppError {
            status: status,
            code: String::from(code),
            message: msg.to_owned(),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        let (status, code, message) = error_code::ErrorCode::from(err).cast();
        AppError {
            status: status,
            code: String::from(code),
            message: String::from(message),
        }
    }
}

impl From<error_code::ErrorCode> for AppError {
    fn from(err: error_code::ErrorCode) -> Self {
        let (status, code, message) = err.cast();
        AppError {
            status: status,
            code: code.to_owned(),
            message: message.to_owned(),
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let api_response = ApiResponse::<String> {
            timestamp: chrono::Utc::now().naive_utc(),
            status: self.status.as_u16(),
            data: None,
            code: Some(self.code),
            message: self.message,
        };

        (self.status, axum::Json(api_response)).into_response()
    }
}
