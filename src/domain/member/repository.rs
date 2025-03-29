use sea_orm::prelude::async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use uuid::{Uuid};
use crate::{
    config::AppContext,
};
use super::entity::MemberEntity;
use super::mapper::MemberMapper;
use super::schema::{Column, Entity as Member, Model as MemberModel};

#[async_trait]
pub trait LoadMemberPort {

    async fn find_by_id(&self, id: Uuid) -> Result<Option<MemberEntity>, sea_orm::DbErr>;

    async fn find_by_email(&self, email: &String) -> Result<Option<MemberEntity>, sea_orm::DbErr>;
}

pub struct MemberQueryRepository {
    ctx: Arc<AppContext>,
}

impl MemberQueryRepository {
    fn new(ctx: Arc<AppContext>) -> Self {
        Self {
            ctx: ctx
        }
    }
}

#[async_trait]
impl LoadMemberPort for MemberQueryRepository {

    async fn find_by_id(&self, id: Uuid) -> Result<Option<MemberEntity>, sea_orm::DbErr> {
        let ormEntity = Member::find_by_id(id)
            .one(&self.ctx.db)
            .await?;
            
        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }

    async fn  find_by_email(&self,  email:  &String) ->  Result<Option<MemberEntity>, sea_orm::DbErr> {
        let ormEntity = Member::find()
            .filter(Column::Email.eq(email))
            .one(&self.ctx.db)
            .await?;

        if ormEntity.is_none() {
            return Ok(None);
        }

        Ok(Some(MemberMapper::to_domain(&ormEntity.unwrap())))
    }
} 
