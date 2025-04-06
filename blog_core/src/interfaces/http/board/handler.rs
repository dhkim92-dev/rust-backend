use std::sync::Arc;

use crate::{
    application::board::{BoardCreateUsecase, BoardDeleteUsecase, BoardDto, BoardModifyUsecase, CreateBoardCommand, ModifyBoardCommand},
    common::{AppError, LoginMember, ReturnValue},
    di::AppContext,
};
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use shaku::HasComponent;

pub async fn create_board(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Json(request): Json<CreateBoardRequest>,
    // ) -> Result<ReturnValue<BoardCommandResponse>, AppError> {
) -> Result<impl IntoResponse, AppError> {
    let command = request.into();
    let board_service: &dyn BoardCreateUsecase = ctx.resolve_ref();
    let board = board_service.create(login_member, command).await?;

    Ok(ReturnValue::new(
        201,
        "게시판이 생성되었습니다.".to_owned(),
        BoardCommandResponse::from(board),
    ))
}

pub async fn update_board(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Json(request): Json<ModifyBoardRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = request.into();
    let usecase: &dyn BoardModifyUsecase = ctx.resolve_ref();
    let board = usecase.modify(login_member, 1, command).await?;

    Ok(ReturnValue::new(
        200,
        "게시판이 수정되었습니다.".to_owned(),
        BoardCommandResponse::from(board),
    ))
}

pub async fn delete_board(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let usecase: &dyn BoardDeleteUsecase = ctx.resolve_ref();
    usecase.delete(login_member, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Serialize)]
pub struct BoardCommandResponse {
    id: i64,
    name: String,
}

impl From<BoardDto> for BoardCommandResponse {
    fn from(board: BoardDto) -> Self {
        BoardCommandResponse {
            id: board.id,
            name: board.name,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateBoardRequest {
    name: String,
}

impl Into<CreateBoardCommand> for CreateBoardRequest {
    fn into(self) -> CreateBoardCommand {
        CreateBoardCommand { name: self.name }
    }
}

#[derive(serde::Deserialize)]
pub struct ModifyBoardRequest {
    name: String,
}

impl Into<ModifyBoardCommand> for ModifyBoardRequest {
    fn into(self) -> ModifyBoardCommand {
        ModifyBoardCommand { name: self.name }
    }
}
