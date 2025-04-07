use std::sync::Arc;

use shaku::Component;
use uuid::Uuid;
use crate::common::error_code::ErrorCode;
use crate::common::{AppError, DbConnProvider, LoginMember};
use crate::domain::board::entity::command::post_entity::PostEntityBuilder;
use crate::domain::board::repository::{LoadBoardPort, LoadPostPort, SavePostPort};

use super::{CreatePostCommand, ModifyPostCommand, PostCreateUsecase, PostDeleteUsecase, PostDto, PostModifyUsecase, PostQueryUsecase, QPostDto};



#[derive(Component)]
#[shaku( interface = PostCreateUsecase ) ]
pub struct PostCreateUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    save_post_port: Arc<dyn SavePostPort>,
}

#[derive(Component)]
#[shaku( interface = PostModifyUsecase )]
pub struct PostModifyUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    load_post_port: Arc<dyn LoadPostPort>,
    #[shaku(inject)]
    save_post_port: Arc<dyn SavePostPort>,
}

#[derive(Component)]
#[shaku( interface = PostDeleteUsecase )]
pub struct PostDeleteUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_post_port: Arc<dyn LoadPostPort>,
    #[shaku(inject)]
    save_post_port: Arc<dyn SavePostPort>,
}

#[derive(Component)]
#[shaku( interface = PostQueryUsecase )]
pub struct PostQueryUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_post_port: Arc<dyn LoadPostPort>,
}

#[async_trait::async_trait]
impl PostCreateUsecase for PostCreateUsecaseImpl {
    async fn create(
        &self,
        login_member: LoginMember,
        command: CreatePostCommand,
    ) -> Result<PostDto, AppError> {

        if !login_member.is_admin() {
            return Err(AppError::from(ErrorCode::Forbidden));
        }

        let new_post = PostEntityBuilder::default()
            .id(None)
            .member_id(login_member.id)
            .category_id(command.category_id)
            .title(command.title)
            .contents(command.contents)
            .created_at(chrono::Utc::now().naive_utc())
            .view_count(0)
            .updated_at(None)
            .build()
            .map_err(|err| {
                tracing::error!("Failed to build post entity: {}", err);
                AppError::with_message(
                    ErrorCode::BadRequest,
                    "Invalid post entity",
                )
            })?;

        let txn = self.db.rw_txn().await?;

        match self.load_board_port.load_entity_by_id(&txn, command.category_id).await {
            Some(board) => board,
            None => {
                return Err(AppError::with_message(
                    ErrorCode::NotFound,
                    "존재하지 않는 게시판입니다.",
                ));
            }
        };

        let post = self.save_post_port.save(&txn, new_post).await;
        txn.commit().await?;

        match post {
            Ok(post) => Ok(PostDto::from(post)),
            Err(err) => {
                tracing::error!("Failed to save post: {}", err);
                Err(AppError::with_message(
                    ErrorCode::InternalServerError,
                    "Failed to save post",
                ))
            }
        }
    }
}

#[async_trait::async_trait]
impl PostModifyUsecase for PostModifyUsecaseImpl {

    async fn update(&self, 
        login_member: LoginMember, 
        id: Uuid, 
        command: ModifyPostCommand
    ) -> Result<PostDto, AppError> {

        let txn = self.db.rw_txn().await?;

        let mut post = self.load_post_port.load_by_id(&txn, id)
            .await 
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::NotFound, "게시글을 찾을 수 없습니다.")
            })?;

        let new_board = self.load_board_port.load_entity_by_id(&txn, command.category_id)
            .await
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::NotFound, "존재하지 않는 게시판입니다.")
            })?;

        post.check_ownership(login_member.id)?;
        post.change_title(command.title);
        post.change_contents(command.contents);
        post.change_category(new_board.get_id().expect("게시판 ID가 없습니다."));

        let saved_post = self.save_post_port.update(&txn, post)
            .await?;
        txn.commit().await?;

        Ok(PostDto::from(saved_post))
    }
}

#[async_trait::async_trait]
impl PostDeleteUsecase for PostDeleteUsecaseImpl {

    async fn delete(&self, login_member: LoginMember, id: Uuid) -> Result<(), AppError> {
        let txn = self.db.rw_txn().await?;
        let post = match self.load_post_port.load_by_id(&txn, id)
            .await {
            Some(post) => post,
            None => {
                return Err(AppError::with_message(
                    ErrorCode::NotFound,
                    "게시글을 찾을 수 없습니다.",
                ));
            }
        };

        post.check_ownership(login_member.id)?;
        self.save_post_port.delete(&txn, id)
            .await?;
        txn.commit().await?;
        Ok(())
    }
} 

#[async_trait::async_trait]
impl PostQueryUsecase for PostQueryUsecaseImpl {
    async fn get_posts(
        &self,
        category_id: Option<i64>,
        cursor: Option<chrono::NaiveDateTime>,
        size: u64,
    ) -> Result<Vec<QPostDto>, AppError> {
        let txn = self.db.rw_txn().await?;
        let posts = self.load_post_port.find_posts(&txn, category_id, cursor, size+1)
            .await
            .unwrap_or(Vec::new());
        txn.commit().await?;

        let posts = posts.into_iter()
            .map(|post| {
                QPostDto::from(post)
            })
            .collect();

        Ok(posts)
    }
}
