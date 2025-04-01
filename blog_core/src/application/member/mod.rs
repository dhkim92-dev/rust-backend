pub mod adapter;

use crate::common::error::error_code::ErrorCode;
use crate::common::middleware::security::LoginMember;
use crate::domain::member::entity::MemberEntity;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shaku::Interface;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberCreateCommand {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberUpdateCommand {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDto {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_activated: bool,
}

impl From<MemberEntity> for MemberDto {
    fn from(member: MemberEntity) -> Self {
        Self {
            id: member.id.unwrap_or_default(),
            nickname: member.nickname,
            email: member.email,
            role: member.role,
            created_at: member.created_at,
            updated_at: member.updated_at,
            is_activated: member.is_activated,
        }
    }
}

#[async_trait::async_trait]
pub trait MemberCreateUseCase: Interface {
    async fn create(&self, command: MemberCreateCommand) -> Result<MemberDto, ErrorCode>;
}

#[async_trait::async_trait]
pub trait MemberUpdateUseCase: Interface {
    async fn update(
        &self,
        login_member: LoginMember,
        command: MemberUpdateCommand,
    ) -> Result<MemberDto, ErrorCode>;
}

#[async_trait::async_trait]
pub trait MemberDeleteUseCase: Interface {
    async fn delete(
        &self,
        login_member: LoginMember,
        target_id: uuid::Uuid,
    ) -> Result<bool, ErrorCode>;
}
