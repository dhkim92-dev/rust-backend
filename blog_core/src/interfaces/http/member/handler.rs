use std::sync::Arc;
use axum::extract::{State, Json, Path};
use crate::application::member::{MemberCreateCommand, MemberCreateUseCase};
use crate::{common::wrapper::ReturnValue, di::AppContext};
use crate::common::error::error_code::ErrorCode;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use shaku::HasComponent;

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
                   Result<ReturnValue<MemberResponse>, ErrorCode>{
    let member_service: &dyn MemberCreateUseCase = ctx.resolve_ref();
    let cmd = MemberCreateCommand {
        nickname: payload.nickname,
        email: payload.email,
        password: payload.password
    };

    Ok(ReturnValue {
        status: 201,
        message: "회원 가입이 완료되었습니다.".to_string(),
        data: MemberResponse {
            id: uuid::Uuid::new_v4(),
            email: "not_implemented".to_string(),
            nickname: "not_implemented".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            role: "not_implemented".to_string(),
            is_activated: false,
        }
    })
}

pub async fn update_member_info(
    State(ctx): State<Arc<AppContext>>, 
    Path(id): Path<uuid::Uuid>, 
    Json(payload): Json<MemberUpdateRequest>) -> 
                   Result<ReturnValue<MemberResponse>, ErrorCode> {
    Ok(ReturnValue {
        status: 200,
        message: "회원 정보가 수정되었습니다.". to_string(),
        data: MemberResponse {
            id: uuid::Uuid::new_v4(),
            email: "not_implemented".to_string(),
            nickname: "not_implemented".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            role: "not_implemented".to_string(),
            is_activated: false
        }
    })
}

pub async fn delete_member(
    State(ctx): State<Arc<AppContext>>,
    Path(id): Path<uuid::Uuid>) -> 
              Result<(), ErrorCode> {
    // Implement the logic to delete a member
    Ok(())
}
