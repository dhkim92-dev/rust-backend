use std::sync::Arc;
use axum::extract::{State, Json, Path};
use crate::di::AppContext;
use crate::common::error::error_code::ErrorCode;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MemberCreateRequest {
    email: String,
    password: String,
    nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberUpdateRequest {
    email: Option<String>,
    password: Option<String>,
    nickname: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberResponse {
    id: uuid::Uuid,
    email: String,
    nickname: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    role: String,   
    is_activated: bool,
}

pub async fn create_new_member(
    State(ctx): State<Arc<AppContext>>, 
    Json(payload): Json<MemberCreateRequest>) -> 
                   Result<Json<MemberResponse>, ErrorCode>{
    Ok(Json(MemberResponse {
        id: uuid::Uuid::new_v4(),
        email: "not_implemented".to_string(),
        nickname: "not_implemented".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        role: "not_implemented".to_string(),
        is_activated: false,
    }))
}

pub async fn update_member_info(
    State(ctx): State<Arc<AppContext>>, 
    Path(id): Path<uuid::Uuid>, 
    Json(payload): Json<MemberUpdateRequest>) -> 
                   Result<Json<MemberResponse>, ErrorCode> {
    Ok(Json(MemberResponse {
        id: uuid::Uuid::new_v4(),
        email: "not_implemented".to_string(),
        nickname: "not_implemented".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        role: "not_implemented".to_string(),
        is_activated: false,
    }))
}

pub async fn delete_member(
    State(ctx): State<Arc<AppContext>>,
    Path(id): Path<uuid::Uuid>) -> 
              Result<(), ErrorCode> {
    // Implement the logic to delete a member
    Ok(())
}
