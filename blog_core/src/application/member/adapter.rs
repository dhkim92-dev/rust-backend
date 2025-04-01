use super::{MemberCreateUseCase, MemberUpdateUseCase, MemberDeleteUseCase, MemberCreateCommand, MemberUpdateCommand, MemberDto};
use shaku::Component;
use std::sync::Arc;
use crate::domain::member::repository::LoadMemberPort;
use crate::common::middleware::security::LoginMember;
use crate::common::error::error_code::ErrorCode;

#[derive(Component)]
#[shaku(interface = MemberCreateUseCase)]
pub struct MemberCreateUseCaseImpl {
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>
}

#[derive(Component)]
#[shaku(interface = MemberUpdateUseCase)]
pub struct MemberUpdateUseCaseImpl {
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>
}

#[derive(Component)]
#[shaku(interface = MemberDeleteUseCase)]
pub struct MemberDeleteUseCaseImpl {
    #[shaku(inject)]
    member_repository: Arc<dyn LoadMemberPort>
}

#[async_trait::async_trait]
impl MemberCreateUseCase for MemberCreateUseCaseImpl {
    
    async fn create(&self, command: MemberCreateCommand) -> Result<MemberDto, ErrorCode> {

        Ok(MemberDto{
            id: uuid::Uuid::new_v4(),
            nickname: command.nickname,
            email: command.email,
            role: "user".to_string(),
            created_at: chrono::Utc::now().to_string(),
            updated_at: chrono::Utc::now().to_string(),
            is_activated: true
        })
    }
}

#[async_trait::async_trait]
impl MemberUpdateUseCase for MemberUpdateUseCaseImpl {
    async fn update(&self, login_member: LoginMember, command: MemberUpdateCommand) -> Result<MemberDto, ErrorCode> {
        Ok(MemberDto{
            id: uuid::Uuid::new_v4(),
            nickname: "not_implemented".to_string(),
            email: "not_implemented".to_string(),
            role: "user".to_string(),
            created_at: chrono::Utc::now().to_string(),
            updated_at: chrono::Utc::now().to_string(),
            is_activated: true
        })
    }
}

#[async_trait::async_trait]
impl MemberDeleteUseCase for MemberDeleteUseCaseImpl {

    async fn delete(&self, login_member: LoginMember, target_id: uuid::Uuid) -> Result<bool, ErrorCode> {
        Ok(true)
    }
}
