mod auth;
mod member;

use std::sync::Arc;
use axum::{extract::State, middleware::{from_fn, from_fn_with_state}, Router};
use sea_orm::DbConn;
use crate::{common::middleware::{envelop::envelop_pattern_middleware, security::jwt_authentication_filter, transaction::transaction_middleware}, di::AppContext};

pub fn create_routers(ctx: Arc<AppContext>, db: DbConn)-> Router {

    Router::new()
        .nest("/api/v1/auth", auth::router(ctx.clone()))
        .nest("/api/v1/members", member::router(ctx.clone()))
        .layer(from_fn_with_state(ctx.clone() ,jwt_authentication_filter))
        .layer(from_fn_with_state(db.clone(), transaction_middleware))
        .layer(from_fn(envelop_pattern_middleware))
        .with_state(db)
}
