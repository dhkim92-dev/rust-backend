use crate::{
    common::{AppError, LoginMember},
    domain::board::entity::{command::board_entity::BoardEntity, query::QBoardEntity},
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

/// 이 구조체는 게시판 목록 조회 시 사용되는 쿼리 전용 DTO입니다.
pub struct QBoardDto {
    /// 게시판 고유 식별자
    pub id: i64,
    /// 게시판 이름
    pub name: String,
    /// 게시판에 속한 게시물 수
    pub count: i64 
}

impl From<QBoardEntity> for QBoardDto {
    fn from(entity: QBoardEntity) -> Self {
        QBoardDto {
            id: entity.id,
            name: entity.name,
            count: entity.count,
        }
    }
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
pub trait BoardQueryUsecase: Interface {
    async fn get_all(&self) -> Result<Vec<QBoardDto>, AppError>;
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
