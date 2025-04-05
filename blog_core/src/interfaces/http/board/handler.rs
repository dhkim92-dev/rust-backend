use std::sync::Arc;

use crate::{
    application::board::{BoardCommandUsecase, BoardDto, CreateBoardCommand},
    common::{AppError, LoginMember, ReturnValue},
    di::AppContext,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use shaku::HasComponent;

pub async fn create_board(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    Json(request): Json<CreateBoardRequest>,
    // ) -> Result<ReturnValue<BoardCommandResponse>, AppError> {
) -> Result<impl IntoResponse, AppError> {
    let command = request.into();
    let board_service: &dyn BoardCommandUsecase = ctx.resolve_ref();
    let board = board_service.create(login_member, command).await?;

    Ok(ReturnValue::new(
        201,
        "게시판이 생성되었습니다.".to_owned(),
        BoardCommandResponse::from(board),
    ))
}

#[derive(serde::Serialize)]
struct BoardCommandResponse {
    id: u64,
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
struct CreateBoardRequest {
    name: String,
}

impl Into<CreateBoardCommand> for CreateBoardRequest {
    fn into(self) -> CreateBoardCommand {
        CreateBoardCommand { name: self.name }
    }
}
