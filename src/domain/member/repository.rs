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
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MemberEntity>, DbErr>;
    async fn find_by_email(&self, email: &String) -> Result<Option<MemberEntity>, DbErr>;
}


#[derive(Component)]
#[shaku(interface = LoadMemberPort)]
pub struct MemberQueryRepository {
    db: DatabaseConnection
}


#[async_trait]
impl LoadMemberPort for MemberQueryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MemberEntity>, DbErr> {
        let ormEntity = Member::find_by_id(id)
            .one(&self.db)
            .await?;
            
        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }

    async fn find_by_email(&self,  email:  &String) ->  Result<Option<MemberEntity>, DbErr> {
        let ormEntity = Member::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await?;

        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }
} 
