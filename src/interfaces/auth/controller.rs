use axum::{
    routing::post, Extension, Json, Router
};
use tracing::info;
use std::sync::{Arc};

use crate::config::AppContext;
use crate::domain::member::repository::MemberQueryRepository;
use crate::{
    interfaces::auth::dto::{LoginRequest, LoginResponse},
    application::dto::auth::{LoginCommand, LoginCommandResponse},
    application::usecases::auth_usecase::{AuthService, AuthUsecase},
};

async fn login(
    Extension(auth_usecase): Extension<Arc<dyn AuthUsecase>>,
    Json(req): Json<LoginRequest>
) -> Result<Json<LoginResponse>, axum::http::StatusCode> {
    info!("Login request: {:?}", req);
    let result = auth_usecase.login_with_email_password(
        LoginCommand {
            principal: req.email.clone(),
            credential: req.password.clone()
        }
    ).await.expect("Login failed");

    Ok(Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: result.access_token,
        refresh_token: result.refresh_token
    }))
}

pub fn router(ctx: Arc<AppContext>) -> Router {
    info!("Creating auth router");

    let member_repository = Arc::new(MemberQueryRepository::new(ctx.clone()));
    let auth_usecase: Arc<dyn AuthUsecase> = Arc::new( AuthService::new(member_repository) );

    info!("Auth Port dependency injected");

    Router::new()
        .route("/api/v1/authentication", post(login))
        .layer(Extension(auth_usecase))
} 
