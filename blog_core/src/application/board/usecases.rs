use crate::{
    common::{AppError, LoginMember},
    domain::board::entity::{command::{board_entity::BoardEntity, post_entity::PostEntity}, query::QBoardEntity},
};
use chrono::NaiveDateTime;
use shaku::Interface;
use uuid::Uuid;

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

pub struct WriterVo {
    pub id: uuid::Uuid,
    pub name: String,
}

pub struct CategoryVo {
    pub id: i64,
    pub name: String
}

pub struct QPostDto {
    pub id: uuid::Uuid,
    pub writer: WriterVo,
    pub category: CategoryVo,
    pub title: String,
    pub contents: String,
    pub created_at: Uuid,
    pub updated_at: Option<Uuid>,
}

/* impl From<PostEntity) for PostDto {
    fn from(entity: PostEntity) -> Self {
        PostDto {
            id: entity.id,
            writer_id: entity.member_id,
            title: entity.title,
            contents: entity.content,
            category_id: entity.category_id,
            updated_at: entity.updated_at.to_string(),
        }
    }
}

impl From<QPostEntity> for QPostDto {
    QPostDto {
        id: entity.id,
        writer: WriterVo {
            id: entity.writer.id,
            name: entity.writer.name,
        },
        category: CategoryVo {
            id: entity.category.id,
            name: entity.category.name,
        },
        title: entity.title,
        contents: entity.content,
        created_at: entity.created_at.to_string(),
        updated_at: entity.updated_at.to_string(),
    }
}
 */

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
    async fn modify(
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

/* 
#[async_trait::async_trait]
pub trait PostQueryUsecase: Interface {
}
*/
