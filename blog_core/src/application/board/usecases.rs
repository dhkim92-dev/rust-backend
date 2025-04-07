use crate::{
    common::{AppError, LoginMember},
    domain::board::entity::{command::{board_entity::BoardEntity, post_entity::PostEntity}, query::{QBoardEntity, QPostEntity}},
};
use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use shaku::Interface;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateBoardCommand {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModifyBoardCommand {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

pub struct CreatePostCommand {
    pub title: String,
    pub contents: String,
    pub category_id: i64,
}

pub struct ModifyPostCommand {
    pub title: String,
    pub contents: String,
    pub category_id: i64,
}

pub struct PostDto {
    pub id: uuid::Uuid,
    pub writer_id: uuid::Uuid,
    pub title: String,
    pub contents: String,
    pub category_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct WriterVo {
    #[sea_orm(from_alias = "writer_id")]
    pub id: uuid::Uuid,
    #[sea_orm(from_alias = "writer_name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct CategoryVo {
    #[sea_orm(from_alias = "category_id")]
    pub id: i64,
    #[sea_orm(from_alias = "category_name")]
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QPostDto {
    pub id: uuid::Uuid,
    pub writer: WriterVo,
    pub category: CategoryVo,
    pub title: String,
    pub contents: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<QPostEntity> for QPostDto {
    fn from(entity: QPostEntity) -> Self {
        QPostDto {
            id: entity.id,
            writer:  entity.writer,
            category: entity.category,
            title: entity.title,
            contents: entity.contents,
            created_at: entity.created_at,
            updated_at: entity.updated_at
        }
    }
}

impl From<PostEntity> for PostDto {
    fn from(entity: PostEntity) -> Self {
        PostDto {
            id: entity.get_id().expect("Id field is required"),
            writer_id: entity.get_member_id(),
            title: entity.get_title(),
            contents: entity.get_contents(),
            category_id: entity.get_category_id(),
            created_at: entity.get_created_at(),
            updated_at: entity.get_updated_at()
        }
    }
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

#[async_trait::async_trait]
pub trait PostCreateUsecase: Interface {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreatePostCommand,
    ) -> Result<PostDto, AppError>;
}

#[async_trait::async_trait]
pub trait PostModifyUsecase: Interface {
    async fn update(
        &self,
        login_member: LoginMember,
        id: Uuid,
        command: ModifyPostCommand,
    ) -> Result<PostDto, AppError>;
}

#[async_trait::async_trait]
pub trait PostDeleteUsecase: Interface {
    async fn delete(
        &self,
        login_member: LoginMember,
        id: Uuid,
    ) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait PostQueryUsecase: Interface {

    async fn get_posts(
        &self,
        category_id: Option<i64>,
        cursor: Option<NaiveDateTime>,
        size: u64,
    ) -> Result<Vec<QPostDto>, AppError>;
}
