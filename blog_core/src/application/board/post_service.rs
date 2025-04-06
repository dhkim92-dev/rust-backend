use std::sync::Arc;

use shaku::Component;
use crate::common::error_code::ErrorCode;
use crate::common::{AppError, DbConnProvider, LoginMember};
use crate::domain::board::entity::command::post_entity::{PostEntity, PostEntityBuilder};
use crate::domain::board::repository::{LoadBoardPort, SavePostPort};

use super::{CreatePostCommand, PostCreateUsecase, PostDeleteUsecase, PostDto, PostModifyUsecase};



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

/* #[derive(Component)]
#[shaku( interface = PostModifyUsecase )]
pub struct PostModifyUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    load_post_port: Arc<dyn SavePostPort>,
    #[shaku(inject)]
    save_post_port: Arc<dyn SavePostPort>,
}

#[derive(Component)]
#[shaku( interface = PostDeleteUsecase )]
pub struct PostDeleteUsecaseImpl{
    #[shaku(inject)]
    db: Arc<dyn DbConnProvider>,
    #[shaku(inject)]
    load_board_port: Arc<dyn LoadBoardPort>,
    #[shaku(inject)]
    load_post_port: Arc<dyn SavePostPort>,
    #[shaku(inject)]
    save_post_port: Arc<dyn SavePostPort>,
} */

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

        let board = match self.load_board_port.load_entity_by_id(&txn, command.category_id).await {
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

/* #[async_trait::async_trait]
impl PostModifyUsecase for PostModifyUsecaseImpl {
}

#[async_trait::async_trait]
impl PostDeleteUsecase for PostDeleteUsecaseImpl {
} */
