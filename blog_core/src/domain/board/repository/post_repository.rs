use sea_orm::DatabaseTransaction;
use shaku::Component;
use shaku::Interface;
use sea_orm::entity::*;
use sea_orm::prelude::*;
use crate::domain::board::entity::command::post_entity::PostEntity;
use crate::domain::board::entity::mapper::post_mapper;
use crate::domain::board::entity::query::QPostEntity;


#[async_trait::async_trait]
pub trait LoadPostPort: Interface {

    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<PostEntity>;

    async fn find_posts(&self, txn: &DatabaseTransaction, category_id: Option<i64>, cursor: Option<Uuid>, size: u64) -> Option<Vec<QPostEntity>>;
}

#[async_trait::async_trait]
pub trait SavePostPort: Interface {

    async fn save(&self, txn: &DatabaseTransaction, post: PostEntity) -> Result<PostEntity, DbErr>;

    async fn update(&self, txn: &DatabaseTransaction, post: PostEntity) -> Result<PostEntity, DbErr>;

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<(), DbErr>;
}

#[derive(Component)]
#[shaku(interface = LoadPostPort)]
pub struct SeaOrmLoadPostAdapter {}

#[derive(Component)]
#[shaku(interface = SavePostPort)]
pub struct SeaOrmSavePostAdapter {
}

#[async_trait::async_trait]
impl LoadPostPort for SeaOrmLoadPostAdapter {
    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<PostEntity> {
        /* post::Entity::find_by_id(id)
            .one(txn)
            .await
            .ok()
            .and_then(|x| x.map(|x| board_mapper::to_domain(&x))) */

        None
    }

    async fn find_posts(&self, txn: &DatabaseTransaction, category_id: Option<i64>, cursor: Option<Uuid>, size: u64) -> Option<Vec<QPostEntity>> {
       /*  let mut query = post::Entity::find();

        if let Some(category_id) = category_id {
            query = query.filter(post::Column::CategoryId.eq(category_id));
        }

        if let Some(cursor) = cursor {
            query = query.filter(post::Column::Id.gt(cursor));
        }

        query.limit(size as u64).all(txn).await */
        None
    }
}

#[async_trait::async_trait]
impl SavePostPort for SeaOrmSavePostAdapter {
    async fn save(&self, txn: &DatabaseTransaction, post: PostEntity) -> Result<PostEntity, DbErr> {
        /* let new_post = post::ActiveModel {
            id: Set(post.id),
            title: Set(post.title),
            content: Set(post.content),
            category_id: Set(post.category_id),
            member_id: Set(post.member_id),
            ..Default::default()
        };*/
        let new_post = post_mapper::to_orm(&post);
        tracing::debug!("new_post: {:?}", new_post);
        new_post.insert(txn).await
            .map_err(|e| {
                tracing::error!("Error inserting post: {:?}", e);
                DbErr::UnpackInsertId
            })
            .map(|x| post_mapper::to_domain(&x))
    }

    async fn update(&self, txn: &DatabaseTransaction, post: PostEntity) -> Result<PostEntity, DbErr> {
        /* let mut active_model = post::ActiveModel {
            id: Set(post.id),
            title: Set(post.title),
            content: Set(post.content),
            category_id: Set(post.category_id),
            member_id: Set(post.member_id),
            ..Default::default()
        };

        active_model.update(txn).await.map(|x| board_mapper::to_domain(&x)) */
        Err(DbErr::UnpackInsertId)
    }

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<(), DbErr> {
        /* let active_model = post::ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        active_model.delete(txn).await */
        Err(DbErr::UnpackInsertId)
    }
}
