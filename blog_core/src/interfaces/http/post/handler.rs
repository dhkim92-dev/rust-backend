use std::sync::Arc;

use axum::extract::State;
use axum::{Extension, Json};
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::application::board::{CreatePostCommand, PostCreateUsecase, PostDto};
use crate::common::{AppError, LoginMember, ReturnValue};
use crate::di::AppContext;
use shaku::HasComponent;


pub async fn create_post(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Json(request): Json<CreatePostRequest>,
) -> Result<ReturnValue<PostCommandResponse>, AppError> {
    let command = request.into();
    let post_service: &dyn PostCreateUsecase = ctx.resolve_ref();
    let post = post_service.create(login_member, command).await?;

    Ok(ReturnValue {
            status: 201,
            data: PostCommandResponse::from(post),
            message: "게시글이 생성되었습니다.".to_owned()
    })
}

#[derive(serde::Deserialize)]
pub struct CreatePostRequest {
    category_id: i64,
    title: String,
    contents: String,
}

impl Into<CreatePostCommand> for CreatePostRequest {
    fn into(self) -> CreatePostCommand {
        CreatePostCommand {
            category_id: self.category_id,
            title: self.title,
            contents: self.contents,
        }
    }
}

#[derive(serde::Serialize)]
pub struct PostCommandResponse {
    id: Uuid,
    category_id: i64,
    writer_id: Uuid,
    title: String,
    contents: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>
}

impl From<PostDto> for PostCommandResponse {
    fn from(post: PostDto) -> Self {
        PostCommandResponse {
            id: post.id,
            category_id: post.category_id,
            writer_id: post.writer_id,
            title: post.title,
            contents: post.contents,
            created_at: post.created_at,
            updated_at: post.updated_at
        }
    }
}
