use super::{
    MemberCreateCommand, MemberCreateUseCase, MemberDeleteUseCase, MemberDto, MemberUpdateCommand,
    MemberUpdateUseCase,
};
use crate::common::database::*;
use crate::common::error::error_code::ErrorCode;
use crate::common::middleware::security::LoginMember;
use crate::domain::member::entity::MemberEntity;
use crate::domain::member::repository::{LoadMemberPort, SaveMemberPort};
use shaku::Component;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = MemberCreateUseCase)]
pub struct MemberCreateUseCaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    save_member_port: Arc<dyn SaveMemberPort>,
}

#[derive(Component)]
#[shaku(interface = MemberUpdateUseCase)]
pub struct MemberUpdateUseCaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    save_member_port: Arc<dyn SaveMemberPort>,
}

#[derive(Component)]
#[shaku(interface = MemberDeleteUseCase)]
pub struct MemberDeleteUseCaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_member_port: Arc<dyn LoadMemberPort>,
    #[shaku(inject)]
    save_member_port: Arc<dyn SaveMemberPort>,
}

#[async_trait::async_trait]
impl MemberCreateUseCase for MemberCreateUseCaseImpl {
    async fn create(&self, command: MemberCreateCommand) -> Result<MemberDto, ErrorCode> {
        let txn = self.db.rw_txn().await?;

        let member = self
            .load_member_port
            .find_by_email(&txn, &command.email)
            .await?;

        if member.is_some() {
            return Err(ErrorCode::EmailAlreadyExists);
        }

        let member_entity = MemberEntity {
            id: Some(uuid::Uuid::new_v4()),
            nickname: command.nickname,
            email: command.email,
            password: bcrypt::hash(command.password, 10).unwrap(),
            role: "MEMBER".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            is_activated: true,
        };

        let member_entity = self.save_member_port.save(&txn, member_entity).await?;
        txn.commit().await?;

        Ok(MemberDto::from(member_entity))
    }
}

#[async_trait::async_trait]
impl MemberUpdateUseCase for MemberUpdateUseCaseImpl {
    async fn update(
        &self,
        login_member: LoginMember,
        resource_id: Uuid,
        command: MemberUpdateCommand,
    ) -> Result<MemberDto, ErrorCode> {
        let txn = self.db.rw_txn().await?;

        if resource_id != login_member.id {
            return Err(ErrorCode::Forbidden);
        }

        let member = self
            .load_member_port
            .find_by_id(&txn, login_member.id)
            .await?;

        if member.is_none() {
            return Err(ErrorCode::MemberNotFound);
        }

        let mut member_entity = member.unwrap();
        member_entity.nickname = command.nickname;
        member_entity.email = command.email;
        member_entity.password = bcrypt::hash(command.password, 10).unwrap();
        member_entity.updated_at = chrono::Utc::now().naive_utc();

        let modified_entity = self.save_member_port.update(&txn, member_entity).await?;
        txn.commit().await?;

        Ok(MemberDto::from(modified_entity))
    }
}

#[async_trait::async_trait]
impl MemberDeleteUseCase for MemberDeleteUseCaseImpl {
    async fn delete(
        &self,
        login_member: LoginMember,
        target_id: uuid::Uuid,
    ) -> Result<bool, ErrorCode> {
        let txn = self.db.rw_txn().await?;
        let member = self.load_member_port.find_by_id(&txn, target_id).await?;

        if member.is_none() {
            return Err(ErrorCode::MemberNotFound);
        }

        //let member_entity = member.unwrap();

        if target_id != login_member.id {
            return Err(ErrorCode::Forbidden);
        }

        self.save_member_port.delete(&txn, target_id).await?;
        txn.commit().await?;

        Ok(true)
    }
}
