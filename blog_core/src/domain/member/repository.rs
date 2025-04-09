use sea_orm::prelude::async_trait::async_trait;
use sea_orm::*;
use super::entity::MemberEntity;
use super::mapper::member_mapper;
use super::schema::{Column, Entity as Member}; //, Model as MemberModel};
use shaku::{Component, Interface};
use uuid::Uuid;

#[async_trait]
pub trait LoadMemberPort: Interface {
    async fn find_by_id(
        &self,
        txn: &DatabaseTransaction,
        id: Uuid,
    ) -> Result<Option<MemberEntity>, DbErr>;

    async fn find_by_email(
        &self,
        txn: &DatabaseTransaction,
        email: &String,
    ) -> Result<Option<MemberEntity>, DbErr>;
}

#[async_trait]
pub trait SaveMemberPort: Interface {
    async fn save(
        &self,
        txn: &DatabaseTransaction,
        member: MemberEntity,
    ) -> Result<MemberEntity, DbErr>;

    async fn update(
        &self,
        txn: &DatabaseTransaction,
        member: MemberEntity,
    ) -> Result<MemberEntity, DbErr>;

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<bool, DbErr>;
}

#[derive(Component)]
#[shaku(interface = LoadMemberPort)]
pub struct MemberQueryRepository {}

#[derive(Component)]
#[shaku(interface = SaveMemberPort)]
pub struct MemberCommandRepository {}

#[async_trait::async_trait]
impl SaveMemberPort for MemberCommandRepository {
    async fn save(
        &self,
        txn: &DatabaseTransaction,
        member: MemberEntity,
    ) -> Result<MemberEntity, DbErr> {
        let orm_entity = member_mapper::to_orm(&member).into_active_model();
        let result = orm_entity.insert(txn).await?;
        Ok(member_mapper::to_domain(&result))
    }

    async fn update(
        &self,
        txn: &DatabaseTransaction,
        member: MemberEntity,
    ) -> Result<MemberEntity, DbErr> {
        let mut orm_entity = member_mapper::to_orm(&member).into_active_model();

        orm_entity.id = Set(member.id.unwrap());
        orm_entity.email = Set(member.email.to_owned());
        orm_entity.nickname = Set(member.nickname.to_owned());
        orm_entity.password = Set(member.password.to_owned());
        orm_entity.role = Set(member.role.to_owned());
        orm_entity.is_activated = Set(member.is_activated.to_owned());
        orm_entity.created_at = Set(member.created_at.to_owned());
        orm_entity.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        let result = orm_entity.update(txn).await?;
        Ok(member_mapper::to_domain(&result))
    }

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<bool, DbErr> {
        match Member::delete_by_id(id).exec(txn).await {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        }
    }
}

#[async_trait]
impl LoadMemberPort for MemberQueryRepository {
    async fn find_by_id(
        &self,
        txn: &DatabaseTransaction,
        id: Uuid,
    ) -> Result<Option<MemberEntity>, DbErr> {
        let orm_entity = Member::find_by_id(id).one(txn).await?;

        if orm_entity.is_none() {
            return Ok(None);
        }

        Ok(Some(member_mapper::to_domain(&orm_entity.unwrap())))
    }

    async fn find_by_email(
        &self,
        txn: &DatabaseTransaction,
        email: &String,
    ) -> Result<Option<MemberEntity>, DbErr> {
        let orm_entity = Member::find()
            .filter(Column::Email.eq(email))
            .one(txn)
            .await?;

        if orm_entity.is_none() {
            return Ok(None);
        }

        Ok(Some(member_mapper::to_domain(&orm_entity.unwrap())))
    }
}
