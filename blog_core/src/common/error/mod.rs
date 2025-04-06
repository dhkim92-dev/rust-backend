use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

use self::error_code::ErrorCode;

use super::ApiResponse;

pub mod error_code;

#[derive(Serialize, Deserialize)]
pub struct AppError {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn with_message(code: ErrorCode, msg: &'static str) -> Self {
        let (status, code, message) = code.cast();
        AppError {
            status: status.as_u16(),
            code: String::from(code),
            message: msg.to_owned(),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        let (status, code, message) = error_code::ErrorCode::from(err).cast();
        AppError {
            status: status.as_u16(),
            code: String::from(code),
            message: String::from(message),
        }
    }
}

impl From<error_code::ErrorCode> for AppError {
    fn from(err: error_code::ErrorCode) -> Self {
        let (status, code, message) = err.cast();
        AppError {
            status: status.as_u16(),
            code: code.to_owned(),
            message: message.to_owned(),
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let api_response = ApiResponse::<String> {
            timestamp: chrono::Utc::now().naive_utc(),
            status: self.status,
            data: None,
            code: Some(self.code),
            message: self.message,
        };

        (StatusCode::from_u16(self.status).unwrap(), Json(api_response)).into_response()
    }
}
