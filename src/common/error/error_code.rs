use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::common::wrapper::ApiResponse;

pub trait ErrorCode: IntoResponse {
    fn error(&self) -> (StatusCode, String, String); // http status, code, message
}


impl IntoResponse for &dyn ErrorCode {
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

