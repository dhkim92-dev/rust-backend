use sea_orm::{DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, Set};
use sea_orm::prelude::*;
use sea_orm::ActiveModelTrait;
use shaku::{Component, Interface};
use crate::common::error_code::ErrorCode;
use crate::common::AppError;

use super::entity::command::board_entity::BoardEntity;
use super::entity::mapper::board_mapper;
use super::schema::board::{Model as BoardModel, Entity as Board};
use std::option::Option;
use std::result::Result;

#[async_trait::async_trait]
pub trait LoadBoardPort: Interface {

    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: u64) -> Option<BoardEntity>;

    // async fn find_by_id(&self, txn: &DatabaseTransaction, id: u64) -> Option<BoardEntity>;

    // async fn find_by_name(&self, txn: &DatabaseTransaction, name: &String) -> Option<BoardEntity>;
}

#[async_trait::async_trait]
pub trait SaveBoardPort: Interface {

    async fn save(&self, txn: &DatabaseTransaction, board: BoardEntity) -> Result<BoardEntity, DbErr>;

    async fn update(&self, txn: &DatabaseTransaction, board: BoardEntity) -> Result<BoardEntity, DbErr>;

    async fn delete(&self, txn: &DatabaseTransaction, id: u64) -> Result<(), DbErr>;
}

#[derive(Component)]
#[shaku(interface = LoadBoardPort)]
pub struct SeaOrmLoadBoardAdapter {}

#[async_trait::async_trait]
impl LoadBoardPort for SeaOrmLoadBoardAdapter {

    async fn load_entity_by_id(&self, txn: &DatabaseTransaction, id: u64) -> Option<BoardEntity> { Board::find_by_id(id)
            .one(txn)
            .await
            .ok()
            .and_then(|x| x.map(|x| board_mapper::to_domain(&x)))
    }

    // async fn find_by_id(&self, txn: &DatabaseTransaction, id: u64) -> Option<BoardEntity> {
        // BoardEntity::find_by_id(txn, id).await
    // }

    // async fn find_by_name(&self, txn: &DatabaseTransaction, name: &String) -> Option<BoardEntity> {
        // BoardEntity::find_by_name(txn, name).awai
    // }
}

#[derive(Component)]
#[shaku(interface = SaveBoardPort)]
pub struct SeaOrmSaveBoardAdapter {
} 

#[async_trait::async_trait]
impl SaveBoardPort for SeaOrmSaveBoardAdapter {

    async fn save(&self, txn: &DatabaseTransaction, board: BoardEntity) -> Result<BoardEntity, DbErr> {
        board_mapper::to_orm(&board).into_active_model()
            .insert(txn)
            .await
            .map(|x| board_mapper::to_domain(&x))
    }

    async fn update(&self, txn: &DatabaseTransaction, board: BoardEntity) -> Result<BoardEntity, DbErr> {

        if board.get_id().is_none() {
            return Err(DbErr::Custom("Primary key not found".to_string()));
        }

        let mut orm_entity = board_mapper::to_orm(&board).into_active_model();
        orm_entity.id = Set(board.get_id().unwrap());
        orm_entity.name = Set(board.get_name());
        orm_entity.created_at = Set(board.get_created_at());
        orm_entity.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        orm_entity.update(txn)
            .await
            .map(|x| board_mapper::to_domain(&x))
    }

    async fn delete(&self, txn: &DatabaseTransaction, id: u64) -> Result<(), DbErr> {
        Board::delete_by_id(id)
            .exec(txn)
            .await
            .map(|_| ())
    } 
}

#[cfg(test)]
mod test {
    use sea_orm::{Database, DatabaseConnection, MockDatabase, MockExecResult, TransactionTrait};
    use sea_orm::entity::*;
    use sea_orm::prelude::*;
    use super::{SeaOrmLoadBoardAdapter,  SeaOrmSaveBoardAdapter,
                LoadBoardPort, SaveBoardPort};
    use crate::domain::board::entity::command::board_entity::BoardEntity;
    use crate::domain::board::schema::board::{Model as BoardModel};


    fn create_mock_db() -> DatabaseConnection {
        MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([
                vec![BoardModel {
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
                    }],
            ]).append_exec_results([
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
        let db:DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmLoadBoardAdapter {};

        let result = adapter.load_entity_by_id(&txn, 1).await.unwrap();
        assert_eq!(result.get_id(), Some(1));
        assert_eq!(result.get_name(), "Test Board");
    }

    #[tokio::test]
    async fn save_test() {
        let db:DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmSaveBoardAdapter {};

        let board = BoardEntity::new(None, 
            "Test Board".to_owned(),
            Some(chrono::Utc::now().naive_utc()),
            None,
        );

        let result = adapter.save(&txn, board)
            .await
            .unwrap();
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
        let db:DatabaseConnection = create_mock_db();
        let txn = db.begin().await.unwrap();
        let adapter = SeaOrmSaveBoardAdapter {};

        let result = adapter.delete(&txn, 1).await;
        txn.commit().await.unwrap();
        assert!(result.is_ok());
    }
}

