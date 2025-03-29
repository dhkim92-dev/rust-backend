use axum::{http::StatusCode, response::IntoResponse, Json};

pub trait ErrorCode {
    fn error(&self) -> (StatusCode, String, String); // http status, code, message
}


