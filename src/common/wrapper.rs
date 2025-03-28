use chrono::{NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CursorList<T> {
    pub count: i32,
    pub data: Vec<T>,
    pub next: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub data: Option<T>,
    pub code: Option<String>,
    pub message: String,
}
