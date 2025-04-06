use sea_orm::prelude::*;
use sea_orm::sea_query::Alias;
use sea_orm::sea_query::Func;
use sea_orm::sea_query::Query;
use sea_orm::ActiveModelTrait;
use sea_orm::Condition;
use sea_orm::JoinType;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{DatabaseTransaction, DbErr, EntityTrait, ColumnTrait, IntoActiveModel, Set};
use shaku::{Component, Interface};

use crate::application::board::QBoardDto;
use crate::domain::board::entity::command::board_entity::BoardEntity;
use crate::domain::board::entity::mapper::board_mapper;
use crate::domain::board::entity::query::QBoardEntity;
use crate::domain::board::schema::board;
use crate::domain::board::schema::post;
use crate::domain::member::schema::Entity;
use std::option::Option;
use std::result::Result;

#[async_trait::async_trait]
pub trait LoadBoardPort: Interface {
    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: i64) -> Option<BoardEntity>;

    async fn find_all(&self,txn: &DatabaseTransaction) -> Result<Vec<QBoardEntity>, DbErr>;

    // async fn find_by_id(&self, txn: &DatabaseTransaction, id: i64) -> Option<BoardEntity>;

    // async fn find_by_name(&self, txn: &DatabaseTransaction, name: &String) -> Option<BoardEntity>;
}

#[async_trait::async_trait]
pub trait SaveBoardPort: Interface {
    async fn save(
        &self,
        txn: &DatabaseTransaction,
        board: BoardEntity,
    ) -> Result<BoardEntity, DbErr>;

    async fn update(
        &self,
        txn: &DatabaseTransaction,
        board: BoardEntity,
    ) -> Result<BoardEntity, DbErr>;

    async fn delete(&self, txn: &DatabaseTransaction, id: i64) -> Result<(), DbErr>;
}

#[derive(Component)]
#[shaku(interface = LoadBoardPort)]
pub struct SeaOrmLoadBoardAdapter {}

#[async_trait::async_trait]
impl LoadBoardPort for SeaOrmLoadBoardAdapter {
    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: i64) -> Option<BoardEntity> {
        board::Entity::find_by_id(id)
            .one(txn)
            .await
            .ok()
            .and_then(|x| x.map(|x| board_mapper::to_domain(&x)))
    }

    async fn find_all(&self, txn: &DatabaseTransaction) -> Result<Vec<QBoardEntity>, DbErr> {
        // SELECT b.id, b.name, count(p) 
        // FROM board b 
        //   LEFT JOIN post p 
        //   ON b.id = p.category_id
        // GROUP BY b.id
        // ORDER BY b.id ASC; 
        // let count_expr = Func::count(Expr::col(post::Column::Id));
        /* let raw_sql = "
            SELECT b.id, b.name, count(p.id) as count 
            FROM article_category b
            LEFT JOIN article p ON b.id = p.category_id
            GROUP BY b.id
            ORDER BY b.id ASC;
        ";

        let query_result: QueryResult =  */

        let result = board::Entity::find()
            .select_only()
            .column(board::Column::Id)
            .column(board::Column::Name)
            .column_as(post::Column::Id.count(), "count")
            .join(
                JoinType::LeftJoin,
                board::Relation::Post.def()
            )
            .group_by(board::Column::Id)
            .order_by_asc(board::Column::Id)
            .into_model::<QBoardEntity>()
            .all(txn)
            .await?;
        tracing::debug!("function name: SeaOrmLoadBoardAdapter::find_all");

        Ok(Vec::from(result))
    }
}

#[derive(Component)]
#[shaku(interface = SaveBoardPort)]
pub struct SeaOrmSaveBoardAdapter {}

#[async_trait::async_trait]
impl SaveBoardPort for SeaOrmSaveBoardAdapter {
    async fn save(
        &self,
        txn: &DatabaseTransaction,
        board: BoardEntity,
    ) -> Result<BoardEntity, DbErr> {
        board_mapper::to_orm(&board)
            .insert(txn)
            .await
            .map(|x| board_mapper::to_domain(&x))
            .map_err(|e| {
                eprintln!("Failed to save board: {}", e);
                e
            })
    }

    async fn update(
        &self,
        txn: &DatabaseTransaction,
        board: BoardEntity,
    ) -> Result<BoardEntity, DbErr> {
        if board.get_id().is_none() {
            return Err(DbErr::Custom("Primary key not found".to_string()));
        }

        let mut orm_entity = board_mapper::to_orm(&board).into_active_model();
        orm_entity.id = Set(board.get_id().unwrap());
        orm_entity.name = Set(board.get_name());
        orm_entity.created_at = Set(board.get_created_at());
        orm_entity.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        orm_entity
            .update(txn)
            .await
            .map(|x| board_mapper::to_domain(&x))
    }

    async fn delete(&self, txn: &DatabaseTransaction, id: i64) -> Result<(), DbErr> {
        board::Entity::delete_by_id(id).exec(txn).await.map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use super::{LoadBoardPort, SaveBoardPort, SeaOrmLoadBoardAdapter, SeaOrmSaveBoardAdapter};
    use crate::domain::board::entity::command::board_entity::BoardEntity;
    use crate::domain::board::schema::board::Model as BoardModel;
    use sea_orm::entity::*;
    use sea_orm::prelude::*;
    use sea_orm::{Database, DatabaseConnection, MockDatabase, MockExecResult, TransactionTrait};

    fn create_mock_db() -> DatabaseConnection {
        MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![
                BoardModel {
                    id: 1,
                    name: "Test Board".to_string(),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: None,
                },
                BoardModel {
                    id: 2,
                    name: "B Board".to_string(),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: Some(chrono::Utc::now().naive_utc()),
                },
            ]])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 0, // 업데이트 쿼리의 결과
                    rows_affected: 1,  // 업데이트된 행 수
                },
            ])
            .into_connection()
    }

    #[tokio::test]
    async fn load_entity_by_id_test() {
        let db: DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmLoadBoardAdapter {};

        let result = adapter.load_entity_by_id(&txn, 1).await.unwrap();
        assert_eq!(result.get_id(), Some(1));
        assert_eq!(result.get_name(), "Test Board");
    }

    #[tokio::test]
    async fn save_test() {
        let db: DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmSaveBoardAdapter {};

        let board = BoardEntity::new(
            None,
            "Test Board".to_owned(),
            Some(chrono::Utc::now().naive_utc()),
            None,
        );

        let result = adapter.save(&txn, board).await.unwrap();
        assert_eq!(result.get_id(), Some(1));
        assert_eq!(result.get_name(), "Test Board");
    }
    /*
       #[tokio::test]
       async fn update_test() {
           let db:DatabaseConnection = create_mock_db();
           let txn = db.begin().await.unwrap();
           let mock_board = BoardEntity::new(
               Some(1),
               "Updated".to_string(),
               Some(chrono::Utc::now().naive_utc()),
               None,
           );
           let adapter = SeaOrmSaveBoardAdapter {};
           let result = adapter.save(&txn, mock_board)
               .await
               .unwrap();

           txn.commit().await.unwrap();

           let txn = db.begin().await.unwrap();
           let load_adapter = SeaOrmLoadBoardAdapter {};
           let mut loaded_board = load_adapter.load_entity_by_id(&txn, 1)
               .await
               .unwrap();
           let _ = loaded_board.change_board_name("Updated Board");

           let result = adapter.update(&txn, loaded_board).await.unwrap();

           txn.commit().await.unwrap();

           assert_eq!(result.get_id(), Some(2));
           assert_eq!(result.get_name(), "Updated");
       }
    */
    #[tokio::test]
    async fn delete_test() {
        let db: DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmSaveBoardAdapter {};

        let result = adapter.delete(&txn, 1).await;
        txn.commit().await.unwrap();
        assert!(result.is_ok());
    }
}
