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
    pub id: i64,
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
pub trait BoardCreateUsecase: Interface {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreateBoardCommand,
    ) -> Result<BoardDto, AppError>;
}

#[async_trait::async_trait]
pub trait BoardModifyUsecase: Interface {
    async fn modify(
        &self,
        login_member: LoginMember,
        id: i64,
        command: ModifyBoardCommand,
    ) -> Result<BoardDto, AppError>;
}

#[async_trait::async_trait]
pub trait BoardDeleteUsecase: Interface {
    async fn delete(
        &self,
        login_member: LoginMember,
        id: i64,
    ) -> Result<(), AppError>;
}

/* #[async_trait::async_trait]
pub trait BoardCommandUsecase: Interface {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreateBoardCommand,
    ) -> Result<BoardDto, AppError>;

    async fn modify(&self, id: i64, command: ModifyCategoryCommand) -> Result<BoardDto, AppError>;
//
    //async fn delete(&self, id: u64) -> Result<(), String>;
} */
