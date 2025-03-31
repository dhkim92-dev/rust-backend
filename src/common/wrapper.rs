use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CursorList<T> {
    pub count: u16,
    pub data: Vec<T>,
    pub next: Option<String>
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub timestamp: NaiveDateTime,
    pub status: u16,
    pub data: Option<T>,
    pub code: Option<String>,
    pub message: String,
}

impl<T: Serialize> ApiResponse <T>{

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

impl <T: Serialize> IntoResponse for ApiResponse<T> {

    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
