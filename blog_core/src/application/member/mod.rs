pub mod adapter;

use serde::{Serialize, Deserialize};
use crate::common::error::error_code::ErrorCode;
use crate::common::middleware::security::LoginMember;
use shaku::{Interface};   

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberCreateCommand {
    pub nickname: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberUpdateCommand {
    pub nickname: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDto {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_activated: bool
}

#[async_trait::async_trait]
pub trait MemberCreateUseCase: Interface {
    async fn create(&self, command: MemberCreateCommand) -> Result<MemberDto, ErrorCode>;
}

#[async_trait::async_trait]
pub trait MemberUpdateUseCase: Interface {
    async fn update(&self, login_member: LoginMember, command: MemberUpdateCommand) -> Result<MemberDto, ErrorCode>;
}

#[async_trait::async_trait]
pub trait MemberDeleteUseCase: Interface {
    async fn delete(&self, login_member: LoginMember, target_id: uuid::Uuid) -> Result<bool, ErrorCode>;
}

