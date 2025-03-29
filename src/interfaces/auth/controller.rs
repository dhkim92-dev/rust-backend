use axum::{
    extract::State, routing::post, Extension, Json, Router
};
use tracing::info;
use std::sync::{Arc};

use crate::interfaces::auth::dto::{
    LoginRequest,
    LoginResponse
};
use crate::application::usecases::auth;

async fn login(
    Extension(auth_usecase): Extension<Arc<dyn AuthUsecase>>,
    Json(req): Json<LoginRequest>
) -> Json<LoginResponse> {
    info!("Login request: {:?}", req);

    let member = auth_service.login_with_email_password(
        crate::application::dto::auth::LoginCommand {
            principal: req.email.address,
            credential: req.password
        }
    ).await.expect("Login failed");

    Json(LoginResponse {
        typ: "Bearer".to_string(),
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    })
}

pub fn router(ctx: Arc<AppContext>) -> Router {
    info!("Creating auth router");

    let member_repository = Arc::new(SeaOrmMemberRepository::new(ctx.clone()));


    Router::new()
        .route("/api/v1/auth", post(login))
} 
