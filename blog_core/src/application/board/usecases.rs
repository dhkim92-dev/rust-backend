use crate::{
    common::{AppError, LoginMember},
    domain::board::entity::command::board_entity::BoardEntity,
};
use shaku::Interface;

pub struct CreateBoardCommand {
    pub name: String,
}

pub struct ModifyBoardCommand {
    pub name: String,
}

pub struct BoardDto {
    pub id: u64,
    pub name: String,
}

impl From<BoardEntity> for BoardDto {
    fn from(entity: BoardEntity) -> Self {
        BoardDto {
            id: entity.get_id().expect("Id field is required"),
            name: entity.get_name(),
        }
    }
}

#[async_trait::async_trait]
pub trait BoardCommandUsecase: Interface {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreateBoardCommand,
    ) -> Result<BoardDto, AppError>;

    //async fn modify(&self, id: u64, command: ModifyCategoryCommand) -> Result<(), AppError>;

    //async fn delete(&self, id: u64) -> Result<(), String>;
}
