use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::error_code::ErrorCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnValue<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CursorList<T> {
    pub count: u16,
    pub data: Vec<T>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub timestamp: NaiveDateTime,
    pub status: u16,
    pub data: Option<T>,
    pub code: Option<String>,
    pub message: String,
}

impl<T: Serialize> ReturnValue<T> {
    pub fn new(status: u16, message: String, data: T) -> Self {
        ReturnValue {
            status,
            message,
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ReturnValue<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let response = ApiResponse::from(self);
        (status, response).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status: u16, message: String, data: Option<T>) -> Self {
        ApiResponse {
            timestamp: Utc::now().naive_utc(),
            status,
            data,
            code: None,
            message,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl<T> From<ReturnValue<T>> for ApiResponse<T> {
    fn from(return_value: ReturnValue<T>) -> Self {
        ApiResponse {
            timestamp: Utc::now().naive_utc(),
            status: return_value.status,
            data: Some(return_value.data),
            code: None,
            message: return_value.message,
        }
    }
}

impl From<ErrorCode> for ApiResponse<()> {
    fn from(error: ErrorCode) -> Self {
        ApiResponse {
            timestamp: Utc::now().naive_utc(),
            status: error.status,
            data: None,
            code: Some(error.code.to_string()),
            message: error.message.to_string(),
        }
    }
}
