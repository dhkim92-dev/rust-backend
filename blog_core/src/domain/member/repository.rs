use sea_orm::prelude::async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::{Uuid};

use super::entity::MemberEntity;
use super::mapper::MemberMapper;
use super::schema::{Column, Entity as Member, Model as MemberModel};
use shaku::{Interface, Component};

#[async_trait]
pub trait LoadMemberPort: Interface {

    async fn find_by_id(&self, txn: &DatabaseTransaction ,id: Uuid) -> Result<Option<MemberEntity>, DbErr>;

    async fn find_by_email(&self, txn: &DatabaseTransaction, email: &String) -> Result<Option<MemberEntity>, DbErr>;
}

#[async_trait]
pub trait SaveMemberPort: Interface {

    async fn save(&self, txn: &DatabaseTransaction, member: MemberEntity) -> Result<MemberEntity, DbErr>;

    async fn update(&self, txn: &DatabaseTransaction, member: MemberEntity) -> Result<MemberEntity, DbErr>;

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<bool, DbErr>;
}


#[derive(Component)]
#[shaku(interface = LoadMemberPort)]
pub struct MemberQueryRepository {
}

#[derive(Component)]
#[shaku(interface = SaveMemberPort)]
pub struct MemberCommandRepository {
}

#[async_trait::async_trait]
impl SaveMemberPort for MemberCommandRepository {

    async fn save(&self, txn: &DatabaseTransaction, member: MemberEntity) -> Result<MemberEntity, DbErr> {
        let mut ormEntity = MemberMapper::to_orm(&member).into_active_model();
        let result = ormEntity.insert(txn).await?;
        Ok(MemberMapper::to_domain(&result))
    }

    async fn update(&self, txn: &DatabaseTransaction, member: MemberEntity) -> Result<MemberEntity, DbErr> {
        let mut ormEntity = MemberMapper::to_orm(&member).into_active_model();
        let result = ormEntity.update(txn).await?;
        Ok(MemberMapper::to_domain(&result))
    }

    async fn delete(&self,  txn: &DatabaseTransaction, id: Uuid) -> Result<bool, DbErr> {
        match Member::delete_by_id(id).exec(txn).await {
            Ok(_) => Ok(true),
            Err(err) => {
                Err(err)
            }
        }
    }
}


#[async_trait]
impl LoadMemberPort for MemberQueryRepository {

    async fn find_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<Option<MemberEntity>, DbErr> {
        let ormEntity = Member::find_by_id(id)
            .one(txn)
            .await?;
            
        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }

    async fn find_by_email(&self,  txn: &DatabaseTransaction, email: &String) ->  Result<Option<MemberEntity>, DbErr> {
        let ormEntity = Member::find()
            .filter(Column::Email.eq(email))
            .one(txn)
            .await?;

        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }
} 
