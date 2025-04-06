pub mod handler;

use axum::routing::put;
use axum::Router;
use axum::{middleware::from_fn, routing::post};
use handler::*;
use std::sync::Arc;

use crate::{common::with_role_admin, di::AppContext};

pub fn router(ctx: Arc<AppContext>) -> axum::Router {
    let board_command_router = axum::Router::new()
        .route(
            "/",
            post(create_board)
        )
        .route(
            "/{id}",
            put(update_board)
        )
        .route( 
            "/{id}",
            axum::routing::delete(delete_board)
        )
        .layer(from_fn(with_role_admin))
        .with_state(ctx.clone());

    let board_query_router = axum::Router::new()
        .route(
            "/",
            axum::routing::get(get_boards_list)
        )
        .with_state(ctx.clone());

    Router::new()
        .merge(board_command_router)
        .merge(board_query_router)
}
