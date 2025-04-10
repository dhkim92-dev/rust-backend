use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use chrono::NaiveDateTime;
use uuid::Uuid;
// use crate::common::error_code::ErrorCode;
use crate::application::board::{CategoryVo, CreatePostCommand, ModifyPostCommand, PostCreateUsecase, PostDeleteUsecase, PostDto, PostModifyUsecase, PostQueryUsecase, QPostDto, WriterVo};
use crate::common::{AppError, CursorList, CursorListBuilder, LoginMember, ReturnValue};
use crate::config::ConfigProvider;
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

pub async fn update_post(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Path(id): Path<Uuid>,
    Json(request): Json<ModifyPostRequest>,
) -> Result<ReturnValue<PostCommandResponse>, AppError> {
    let command: ModifyPostCommand = request.into();
    let post_service: &dyn PostModifyUsecase = ctx.resolve_ref();
    let post = post_service.update(login_member, id, command).await?;

    Ok(ReturnValue {
            status: 200,
            data: PostCommandResponse::from(post),
            message: "게시글이 수정되었습니다.".to_owned()
    })
}

pub async fn delete_post(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let post_delete_usecase: &dyn PostDeleteUsecase = ctx.resolve_ref();
    post_delete_usecase.delete(login_member, id).await?;
    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

pub async fn get_posts(
    State(ctx): State<Arc<AppContext>>,
    Query(params): Query<PostQueryParams>,
) -> Result<ReturnValue<CursorList<PostQueryResponse>>, AppError> {
    //let size  = size.unwrap_or(20);
    let size = params.size.unwrap_or(20);
    let category_id = params.category_id;
    let created_at = params.created_at;
    let post_service: &dyn PostQueryUsecase = ctx.resolve_ref();
    let config_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let posts = post_service.get_posts(category_id, created_at, size as u64).await?;

    let posts = posts.into_iter()
        .map(PostQueryResponse::from)
        .collect::<Vec<PostQueryResponse>>();

    let mut cursor_list_builder = CursorListBuilder::new(posts, size)
        .set_target("created_at".to_owned());

    if let Some(category_id) = category_id {
        cursor_list_builder = cursor_list_builder.register_query("category_id".to_owned(), category_id.to_string());
    }

    Ok(ReturnValue {
        status: 200,
        data: cursor_list_builder.build(config_provider.get_uri("/api/v1/posts")),
        message: "게시글 목록을 가져왔습니다.".to_owned()
    })
}

pub async fn get_post(
    State(ctx): State<Arc<AppContext>>,
    Path(id): Path<Uuid>,
) -> Result<ReturnValue<PostQueryResponse>, AppError> {
    let query_usecase: &dyn PostQueryUsecase = ctx.resolve_ref();

    let post = query_usecase.get_post(id).await?;

    Ok(ReturnValue {
        status: 200,
        data: PostQueryResponse::from(post),
        message: "게시글을 가져왔습니다.".to_owned()
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


#[derive(serde::Deserialize)]
pub struct ModifyPostRequest {
    category_id: i64,
    title: String,
    contents: String,
}

impl Into<ModifyPostCommand> for ModifyPostRequest {
    fn into(self) -> ModifyPostCommand {
        ModifyPostCommand {
            title: self.title,
            contents: self.contents,
            category_id: self.category_id
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PostQueryResponse {
    id: Uuid,
    writer: WriterVo,
    category: CategoryVo,
    title: String,
    contents: Option<String>,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>
}

impl From<QPostDto> for PostQueryResponse {
    fn from(post: QPostDto) -> Self {
        PostQueryResponse {
            id: post.id,
            writer: post.writer,
            category: post.category,
            title: post.title,
            contents: post.contents,
            created_at: post.created_at,
            updated_at: post.updated_at
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PostQueryParams {
    category_id: Option<i64>,
    created_at: Option<NaiveDateTime>,
    size: Option<usize>,
}
