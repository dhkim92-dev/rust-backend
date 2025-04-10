// use sea_orm::sea_query::extension::postgres::PgExpr;
// use sea_orm::sea_query::Func;
use sea_orm::DatabaseTransaction;
use sea_orm::JoinType;
use shaku::Component;
use shaku::Interface;
use sea_orm::entity::*;
use sea_orm::prelude::*;
use sea_orm::*;
use chrono::NaiveDateTime;
use crate::domain::board::entity::command::post_entity::PostEntity;
use crate::domain::board::entity::mapper::post_mapper;
use crate::domain::board::entity::query::QPostEntity;
use crate::domain::board::schema::post;
use crate::domain;


#[async_trait::async_trait]
pub trait LoadPostPort: Interface {

    async fn load_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<PostEntity>;

    async fn find_posts(&self, txn: &DatabaseTransaction, category_id: Option<i64>, cursor: Option<NaiveDateTime>, size: u64) -> Option<Vec<QPostEntity>>;

    async fn find_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<QPostEntity>;
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

    async fn load_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<PostEntity> {
        match post::Entity::find_by_id(id).one(txn).await {
            Ok(Some(post)) => Some(post_mapper::to_domain(&post)),
            Ok(None) => None,
            Err(e) => {
                tracing::error!("Error loading post by id: {:?}", e);
                None
            }
        }
    }

    async fn find_posts(&self, txn: &DatabaseTransaction, category_id: Option<i64>, cursor: Option<NaiveDateTime>, size: u64) -> Option<Vec<QPostEntity>> {

        let filter_expr = if let Some(category_id) = category_id {
            post::Column::CategoryId.eq(category_id)
        } else {
            post::Column::CategoryId.is_not_null()
        };

        let filter_expr = if let Some(cursor) = cursor {
            filter_expr.and(post::Column::CreatedAt.lte(cursor))
        } else {
            filter_expr.and(post::Column::CreatedAt.lte(chrono::Utc::now().naive_utc()))
        };

        let result = post::Entity::find()
            .select_only()
            .column(post::Column::Id)
            .column_as(post::Column::MemberId, "writer_id")
            .column_as(domain::member::schema::Column::Nickname, "writer_name")
            .column_as(post::Column::CategoryId, "category_id")
            .column_as(domain::board::schema::board::Column::Name, "category_name")
            .column(post::Column::Title)
            .column(post::Column::Contents)
            .column(post::Column::CreatedAt)
            .column(post::Column::UpdatedAt)
            .join(
                JoinType::LeftJoin, 
                post::Relation::Member.def()
            )
            .join(
                JoinType::LeftJoin, 
                post::Relation::Board.def()
            )
            .filter(filter_expr)
            .limit(size)
            .order_by_desc(post::Column::CreatedAt)
            .into_model::<QPostEntity>()
            .all(txn)
            .await;

        match result {
            Ok(posts) => {
                Some(posts)
            }
            Err(e) => {
                tracing::error!("Error finding posts: {:?}", e);
                None
            }
        }
    }

    async fn find_by_id(&self, txn: &DatabaseTransaction, id: Uuid) -> Option<QPostEntity> {
        let result = post::Entity::find()
            .select_only()
            .column(post::Column::Id)
            .column_as(post::Column::MemberId, "writer_id")
            .column_as(domain::member::schema::Column::Nickname, "writer_name")
            .column_as(post::Column::CategoryId, "category_id")
            .column_as(domain::board::schema::board::Column::Name, "category_name")
            .column(post::Column::Title)
            .column(post::Column::Contents)
            .column(post::Column::CreatedAt)
            .column(post::Column::UpdatedAt)
            .join(
                JoinType::LeftJoin, 
                post::Relation::Member.def()
            )
            .join(
                JoinType::LeftJoin, 
                post::Relation::Board.def()
            )
            .filter(post::Column::Id.eq(id))
            .into_model::<QPostEntity>()
            .one(txn)
            .await;

        tracing::debug!("find_by_id result: {:?}", result);

        match result {
            Ok(Some(post)) => Some(post),
            Ok(None) => None,
            Err(e) => {
                tracing::error!("Error finding post by id: {:?}", e);
                None
            }
        }
    }
}

#[async_trait::async_trait]
impl SavePostPort for SeaOrmSavePostAdapter {
    async fn save(&self, txn: &DatabaseTransaction, post: PostEntity) -> Result<PostEntity, DbErr> {
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
        let mut active_model = post_mapper::to_orm(&post);
        active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        active_model.update(txn).await
            .map_err(|e| {
                tracing::error!("Error updating post: {:?}", e);
                DbErr::UnpackInsertId
            })
            .map(|x| post_mapper::to_domain(&x))
    }

    async fn delete(&self, txn: &DatabaseTransaction, id: Uuid) -> Result<(), DbErr> {
        let _  = post::Entity::delete_by_id(id)
            .exec(txn)
            .await
            .map_err(|e| {
                tracing::error!("Error deleting post: {:?}", e);
                DbErr::UnpackInsertId
            });

        Ok(())
    }
}
