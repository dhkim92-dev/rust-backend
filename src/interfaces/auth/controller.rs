use axum::{
    routing::post, Extension, Json, Router
};
use tracing::info;
use std::sync::{Arc};

use crate::config::AppContext;
use crate::{
    interfaces::auth::dto::{LoginRequest, LoginResponse},
    application::dto::auth::{LoginCommand, LoginCommandResponse},
    application::usecases::auth_usecase::{AuthService, AuthUsecase},
    // domain::member::repository::{MemberQueryRepository, LoadMemberPort},
};

async fn login(
    Extension(auth_usecase): Extension<Arc<dyn AuthUsecase>>,
    Json(req): Json<LoginRequest>
) -> Result<Json<LoginResponse>, axum::http::StatusCode> {
    info!("Login request: {:?}", req);

    let member = auth_usecase.login_with_email_password(
        LoginCommand {
            principal: req.email.address,
            credential: req.password
        }
    ).await.expect("Login failed");

    Ok(Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    }))
}

pub fn router(ctx: Arc<AppContext>) -> Router {
    info!("Creating auth router");

    // let member_repository = Arc::new(MemberQueryRepository::new(ctx.clone()));
    // let auth_usecase = Arc::new( AuthService::new(member_repository) );

    Router::new()
        // .route("/api/v1/auth", post(login))
        // .layer(Extension(auth_usecase))
} 
