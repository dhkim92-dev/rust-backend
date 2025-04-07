use crate::application::member::*;
use crate::common::error::error_code::ErrorCode;
use crate::common::middleware::security::LoginMember;
use crate::{common::wrapper::ReturnValue, di::AppContext};
use axum::extract::{Extension, Json, Path, State};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shaku::HasComponent;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct MemberCreateRequest {
    email: String,
    password: String,
    nickname: String,
}

impl Into<MemberCreateCommand> for MemberCreateRequest {
    fn into(self) -> MemberCreateCommand {
        MemberCreateCommand {
            email: self.email,
            password: self.password,
            nickname: self.nickname,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemberUpdateRequest {
    email: Option<String>,
    password: Option<String>,
    nickname: Option<String>,
}

impl Into<MemberUpdateCommand> for MemberUpdateRequest {
    fn into(self) -> MemberUpdateCommand {
        MemberUpdateCommand {
            email: self.email.expect("Email is required"),
            password: self.password.expect("Password is required"),
            nickname: self.nickname.expect("Nickname is required"),
        }
    }
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

impl From<MemberDto> for MemberResponse {
    fn from(member: MemberDto) -> Self {
        Self {
            id: member.id,
            email: member.email,
            nickname: member.nickname,
            created_at: member.created_at,
            updated_at: member.updated_at,
            role: member.role,
            is_activated: member.is_activated,
        }
    }
}

pub async fn create_new_member(
    State(ctx): State<Arc<AppContext>>,
    Json(payload): Json<MemberCreateRequest>,
) -> Result<ReturnValue<MemberResponse>, ErrorCode> {
    let member_service: &dyn MemberCreateUseCase = ctx.resolve_ref();
    let cmd = payload.into();
    let member_dto = member_service.create(cmd).await?;

    Ok(ReturnValue {
        status: 201,
        message: "회원 가입이 완료되었습니다.".to_string(),
        data: MemberResponse::from(member_dto),
    })
}

pub async fn update_member_info(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<MemberUpdateRequest>,
) -> Result<ReturnValue<MemberResponse>, ErrorCode> {
    let member_update_usecase: &dyn MemberUpdateUseCase = ctx.resolve_ref();
    let member_dto = member_update_usecase
        .update(login_member, id, payload.into())
        .await?;

    Ok(ReturnValue {
        status: 200,
        message: "회원 정보가 수정되었습니다.".to_string(),
        data: MemberResponse::from(member_dto),
    })
}

pub async fn delete_member(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Path(id): Path<uuid::Uuid>,
) -> Result<axum::response::Response, ErrorCode> {
    let member_delete_usecase: &dyn MemberDeleteUseCase = ctx.resolve_ref();
    member_delete_usecase.delete(login_member, id).await?;
    Ok(axum::response::Response::builder()
        .status(204)
        .body(axum::body::Body::empty())
        .unwrap())
}
