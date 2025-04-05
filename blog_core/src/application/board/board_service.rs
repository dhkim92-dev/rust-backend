use crate::{
    common::{error_code::ErrorCode, AppError, DbConnProvider, LoginMember},
    domain::board::{
        entity::command::board_entity::BoardEntity,
        repository::{LoadBoardPort, SaveBoardPort},
    },
};
use shaku::Component;
use std::sync::Arc;

use super::{BoardCommandUsecase, BoardDto, CreateBoardCommand};

#[derive(Component)]
#[shaku(interface = BoardCommandUsecase)]
pub struct BoardCommandService {
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    save_board_port: Arc<dyn SaveBoardPort>,
}

#[async_trait::async_trait]
impl BoardCommandUsecase for BoardCommandService {
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
