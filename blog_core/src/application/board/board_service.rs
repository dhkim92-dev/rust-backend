use crate::{
    common::{error_code::ErrorCode, AppError, DbConnProvider, LoginMember},
    domain::board::{
        entity::command::board_entity::BoardEntity,
        repository::{LoadBoardPort, SaveBoardPort},
    },
};
use shaku::Component;
use std::sync::Arc;

use super::{BoardCreateUsecase, BoardDeleteUsecase, BoardDto, BoardModifyUsecase, CreateBoardCommand, ModifyBoardCommand};

#[derive(Component)]
#[shaku(interface = BoardCreateUsecase)]
pub struct BoardCreateUsecaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    save_board_port: Arc<dyn SaveBoardPort>,
}

#[derive(Component)]
#[shaku(interface = BoardModifyUsecase)]
pub struct BoardModifyUsecaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    save_board_port: Arc<dyn SaveBoardPort>,
}

#[derive(Component)]
#[shaku(interface = BoardDeleteUsecase)]
pub struct BoardDeleteUsecaseImpl {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    save_board_port: Arc<dyn SaveBoardPort>,
}

#[async_trait::async_trait]
impl BoardCreateUsecase for BoardCreateUsecaseImpl {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreateBoardCommand,
    ) -> Result<BoardDto, AppError> {
        if login_member.role != "ROLE_ADMIN" {
            return Err(AppError::with_message(
                ErrorCode::Forbidden,
                "You are not authorized to create a board",
            ));
        }
        let txn = self.db.rw_txn().await?;
        let board = BoardEntity::new(None, command.name, None, None);
        let board = self.save_board_port.save(&txn, board).await?;
        txn.commit().await;

        Ok(BoardDto::from(board))
    }
}

#[async_trait::async_trait]
impl BoardModifyUsecase for BoardModifyUsecaseImpl {
    async fn modify(
        &self,
        login_member: LoginMember,
        id: i64,
        command: ModifyBoardCommand,
    ) -> Result<BoardDto, AppError> {
        if login_member.role != "ROLE_ADMIN" {
            return Err(AppError::with_message(
                ErrorCode::Forbidden,
                "You are not authorized to modify a board",
            ));
        }

        let txn = self.db.rw_txn().await?;
        let board = self.load_board_port.load_entity_by_id(&txn, id).await;

        if board.is_none() {
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                "Board not found",
            ));
        }

        let mut board: BoardEntity = board.expect("Board not found");
        board.change_board_name(command.name.as_str())?;
        let board = self.save_board_port.update(&txn, board).await?;
        txn.commit().await;

        Ok(BoardDto::from(board))
    }
}

#[async_trait::async_trait]
impl BoardDeleteUsecase for BoardDeleteUsecaseImpl {
    async fn delete(&self, login_member: LoginMember, id: i64) -> Result<(), AppError> {
        if login_member.role != "ROLE_ADMIN" {
            return Err(AppError::with_message(
                ErrorCode::Forbidden,
                "You are not authorized to delete a board",
            ));
        }
        let txn = self.db.rw_txn().await?;
        let board = self.load_board_port.load_entity_by_id(&txn, id).await;

        if board.is_none() {
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                "Board not found",
            ));
        }

        let board = board.unwrap();
        self.save_board_port.delete(&txn, board.get_id().unwrap()).await?;
        txn.commit().await;

        Ok(())
    }
}
